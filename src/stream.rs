use crate::{
    error::SurgeError,
    sdk::SurgeSdk,
    types::{Auth, Event},
};
use bytes::Bytes;
use flate2::{Compression, write::GzEncoder};
use futures_util::{Stream, StreamExt};
use ignore::{WalkBuilder, gitignore::GitignoreBuilder};
use log::{debug, error, info, trace};
use ndjson_stream::{
    config::{EmptyLineHandling, NdjsonConfig},
    fallible::FallibleNdjsonError,
};
use reqwest::Body;
use serde_json::{Value, json};
use std::os::unix::fs::PermissionsExt;
use std::pin::Pin;
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};
use tar::{Builder, Header};
use thiserror::Error;
use tokio::io::{AsyncWriteExt, DuplexStream};
use tokio::task::JoinHandle;
use tokio_util::io::ReaderStream;

/// Errors that can occur during tarbar creation or directory traversal.
#[derive(Debug, Error)]
enum TarGzError {
    /// Wraps I/O errors from file operations.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Wraps errors from directory traversal using `ignore` crate.
    #[error("Directory walk error: {0}")]
    WalkError(#[from] ignore::Error),
}

/// A stream that generates a `.tar.gz` archive of a project directory
struct TarGzStream {
    reader: ReaderStream<DuplexStream>, // Stream for reading tarball chunks
    task: Option<JoinHandle<Result<(), SurgeError>>>, // Async task for tarbar creation
    done: bool,                         // Flag to indicate stream completition
}

/// Metadata about a project directory, including file count and total size.
#[derive(Debug, Clone)]
pub struct StreamMetadata {
    pub file_count: u64,   // Numbers of files in project
    pub project_size: u64, // Total size of files in bytes
}

/// Calculates metadata (file count and total size) for a project directory.
///
/// # Arguments
/// * `project_path` - Path to the project directory.
///
/// # Returns
/// A `Result` containing `StreamMetadata` or a `SurgeError` if the path is invalid or an error occurs.
///
/// # Notes
/// - Respects `.surgeignore` rules for excluding files.
/// - Uses parallel directory traversal for efficiency.
pub fn calculate_metadata(project_path: &Path) -> Result<StreamMetadata, SurgeError> {
    debug!("Calculating metadata for path: {:?}", project_path);

    // Validate that the path is a directory
    if !project_path.is_dir() {
        error!("Project path {:?} is not a directory", project_path);
        return Err(SurgeError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Invalid project directory",
        )));
    }

    // Build gitignore rules for filtering
    let gitignore = build_custom_gitignore(project_path)?;

    // Set up parallel di traversal
    let walker = WalkBuilder::new(project_path)
        .standard_filters(false) // Disable default .gitignore filters
        .build_parallel();

    // Channel for ccollecting directoty entries
    let (tx, rx) = std::sync::mpsc::channel();

    // Run the directory walker
    walker.run(move || {
        let tx = tx.clone();
        let gitignore = gitignore.clone();

        Box::new(move |result| {
            match result {
                Ok(entry) => {
                    let path = entry.path();
                    // Check if the path is ignored
                    let matched = gitignore.matched_path_or_any_parents(path, path.is_dir());
                    if !matched.is_ignore() {
                        tx.send(entry).ok(); // Send non-ignored entries
                    };
                }
                Err(err) => {
                    error!("Walker error: {:?}", err);
                }
            }
            ignore::WalkState::Continue
        })
    });

    let mut file_count = 0;
    let mut project_size = 0;

    // Process entries for calculate metadata
    for entry in rx {
        let path = entry.path();
        trace!("Processing file for metadata: {:?}", path);
        if path.is_file() {
            let metadata = fs::metadata(path).map_err(SurgeError::Io)?;
            file_count += 1;
            project_size += metadata.len();
            debug!("Counted file: {:?}: {} bytes", path, metadata.len());
        }
    }

    debug!(
        "Metadata calculated: {} files, {} bytes",
        file_count, project_size
    );

    Ok(StreamMetadata {
        file_count,
        project_size,
    })
}

/// Creates a new `TarGzStream` for a project directory.
///
/// # Arguments
/// * `project_path` - Path to the project directory.
/// * `chunk_size` - Size of the duplex stream buffer.
///
/// # Returns
/// A `Result` containing the `TarGzStream` or a `SurgeError` if the path is invalid or an error occurs.
impl TarGzStream {
    fn new(project_path: &Path, chunk_size: usize) -> Result<Self, SurgeError> {
        debug!("Creating new TarGzStream for path: {:?}", project_path);

        // Validate that the path is a directory
        if !project_path.is_dir() {
            error!("Project path {:?}: is not a directory", project_path);
            return Err(SurgeError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Invalid project directory",
            )));
        }

        // Extract directory name for tarball paths
        let dir_name = project_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("project")
            .to_string();

        let project_path = project_path.to_path_buf();
        let ignore_matcher = build_custom_gitignore(&project_path)?;

        // Create a duplex stream for async I/O
        let (reader, writer) = tokio::io::duplex(chunk_size);

        // Spawn an async task to build the tarball
        let task = tokio::spawn(async move {
            // Temporary buffer for tarball data
            let buffer = Vec::new();
            let mut encoder = GzEncoder::new(buffer, Compression::new(6));

            // Build tar in a block to drop it before encoder.finish()
            {
                let mut tar = Builder::new(&mut encoder);

                let walker = WalkBuilder::new(&project_path)
                    .standard_filters(false)
                    .build();

                for entry in walker {
                    let entry = entry.map_err(SurgeError::IgnoreError)?;
                    let path = entry.path();

                    // Skip ignored files or non-files
                    let is_ignored = ignore_matcher
                        .matched_path_or_any_parents(path, path.is_dir())
                        .is_ignore();

                    if is_ignored || !path.is_file() {
                        trace!("Ignored or not a file: {}", path.display());
                        continue;
                    }

                    // Process each file
                    if path.is_file() {
                        trace!("Processing file: {}", path.display());

                        // Compute relative path for tar
                        let rel_path = path
                            .strip_prefix(project_path.parent().unwrap_or(Path::new("")))
                            .map_err(|e| {
                                SurgeError::Io(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    e.to_string(),
                                ))
                            })?;
                        // Get file_name and handle None case
                        let file_name = rel_path.file_name().ok_or_else(|| {
                            SurgeError::Io(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                format!("No file name for path: {}", path.display()),
                            ))
                        })?;

                        let tar_path = PathBuf::from(&dir_name).join(file_name);
                        let metadata = fs::metadata(path)?;
                        debug!(
                            "Adding file to tar: {} (size: {}, mode: {:o})",
                            tar_path.display(),
                            metadata.len(),
                            metadata.permissions().mode()
                        );

                        // Set up tar header
                        let mut header = Header::new_ustar();
                        header.set_size(metadata.len());
                        header.set_mode(0o644); // Standard file permissions
                        header.set_mtime(
                            metadata
                                .modified()
                                .map(|t| t.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs())
                                .unwrap_or(0),
                        );
                        header.set_cksum();

                        // Add file to tar
                        let mut file = File::open(path)?;
                        tar.append_data(&mut header, &tar_path, &mut file)
                            .map_err(|e| SurgeError::Io(std::io::Error::other(e.to_string())))?;
                    }
                }

                tar.finish()?;
            } // Drop tar to release encoder borrow

            // Finalize gzip compression
            let data = encoder.finish()?;

            // Write tarball to the duplex stream
            let mut writer = writer;
            writer.write_all(&data).await?;
            writer.shutdown().await?;
            Ok(())
        });

        Ok(Self {
            reader: ReaderStream::new(reader),
            task: Some(task), // Wrap task in Some
            done: false,
        })
    }
}

/// Implements the `Stream` trait to produce chunks of the `.tar.gz` archive.
impl Stream for TarGzStream {
    type Item = Result<Bytes, SurgeError>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.done {
            debug!("TarGzStream is done, returning None");
            return std::task::Poll::Ready(None);
        }

        // Poll the tarball creation task if it exists
        if let Some(task) = self.task.as_mut() {
            match futures_util::ready!(Pin::new(task).poll(cx)) {
                Ok(Ok(())) => {
                    self.task = None; // Clear the task to prevent re-polling
                    debug!("Tarball creation task completed successfully");
                }
                Ok(Err(e)) => {
                    error!("Tarball creation failed: {}", e);
                    self.task = None; // Clear the task
                    self.done = true;
                    return std::task::Poll::Ready(Some(Err(e)));
                }
                Err(e) => {
                    error!("Task panicked: {}", e);
                    self.task = None; // Clear the task
                    self.done = true;
                    return std::task::Poll::Ready(Some(Err(SurgeError::Io(
                        std::io::Error::other(e.to_string()),
                    ))));
                }
            }
        }

        // Poll the reader for chunks
        match Pin::new(&mut self.reader).poll_next(cx) {
            std::task::Poll::Ready(Some(Ok(bytes))) => {
                debug!("Returning chunk of {} bytes", bytes.len());
                std::task::Poll::Ready(Some(Ok(bytes)))
            }
            std::task::Poll::Ready(Some(Err(e))) => {
                error!("Stream read error: {}", e);
                self.done = true;
                std::task::Poll::Ready(Some(Err(SurgeError::Io(e))))
            }
            std::task::Poll::Ready(None) => {
                debug!("Stream is complete");
                self.done = true;
                std::task::Poll::Ready(None)
            }
            std::task::Poll::Pending => {
                trace!("Stream is pending");
                std::task::Poll::Pending
            }
        }
    }
}

/// Publishes a project directory as a `.tar.gz` archive to a remote endpoint.
///
/// # Arguments
/// * `client` - The `SurgeSdk` client for making HTTP requests.
/// * `project_path` - Path to the project directory.
/// * `domain` - Target domain for publishing.
/// * `auth` - Authentication credentials.
/// * `headers` - Optional custom HTTP headers.
/// * `argv` - Optional command-line arguments for the request.
///
/// # Returns
/// A `Result` containing a stream of `Event`s or a `SurgeError` if the request fails.
pub async fn publish(
    client: &SurgeSdk,
    project_path: &Path,
    domain: &str,
    auth: &Auth,
    headers: Option<Vec<(String, String)>>,
    argv: Option<&[String]>,
) -> Result<impl Stream<Item = Result<Event, SurgeError>>, SurgeError> {
    info!("Publishing to domain: {}", domain);
    debug!("Project path: {:?}", project_path);

    // Construct the target URL
    let url = format!("{}{}", client.config.endpoint, domain);
    debug!("URL: {}", url);

    // Calculate project metadata
    let metadata = calculate_metadata(project_path)?;
    let timestamp = chrono::Utc::now().to_rfc3339();

    // Serialize argv as JSON
    let argv_json = argv.map_or_else(
        || {
            Ok(json!({
                "_": [],
                "e": client.config.endpoint.as_str(),
                "endpoint": client.config.endpoint.as_str(),
                "s": false,
                "stage": false
            })
            .to_string())
        },
        |args| {
            serde_json::to_string(&json!({
                "_": args,
                "e": client.config.endpoint.as_str(),
                "endpoint": client.config.endpoint.as_str(),
                "s": false,
                "stage": false
            }))
        },
    )?;

    // Build HTTP request
    let mut req = client
        .client
        .put(&url)
        .header("Content-Type", "application/gzip")
        .header("Accept", "application/ndjson")
        .header("version", &client.config.version)
        .header("timestamp", timestamp)
        .header("stage", "false")
        .header("ssl", "null")
        .header("argv", argv_json);

    req = req
        .header("file-count", metadata.file_count.to_string())
        .header("project-size", metadata.project_size.to_string());

    // Add custom headers if provided
    if let Some(headers) = headers {
        debug!("Adding custom headers: {:?}", headers);
        for (key, value) in headers {
            req = req.header(&key, value);
        }
    }

    // Create tarball stream and attach to request
    let tar_gz_stream = TarGzStream::new(project_path, 8192)?;
    req = req.body(Body::wrap_stream(tar_gz_stream));
    req = client.apply_auth(req, auth);

    // Send request
    debug!("Sending request to {}", url);
    let res = req.send().await?;
    debug!("Response status: {}", res.status());

    // Check response status
    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await?;
        error!("Request failed with status {}: {}", status, text);
        return Err(SurgeError::Api(crate::error::ApiError {
            errors: vec![format!("Request failed with status: {}", status)],
            details: Value::Object(serde_json::Map::new()),
            status: Some(status.as_u16()),
        }));
    }

    info!("Successfully uploaded tarball for domain: {}", domain);

    // Process response as NDJSON stream
    let stream = res.bytes_stream();
    let config = NdjsonConfig::default().with_empty_line_handling(EmptyLineHandling::IgnoreEmpty);

    let stream = stream.map(|result| {
        result.map_err(SurgeError::from).and_then(|bytes| {
            trace!("Received {} bytes", bytes.len());
            String::from_utf8(bytes.to_vec()).map_err(|e| {
                error!("UTF-8 error: {}", e);
                SurgeError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            })
        })
    });

    let ndjson_stream = ndjson_stream::from_fallible_stream_with_config(stream, config);

    Ok(ndjson_stream.map(|result: Result<Event, _>| match result {
        Ok(event) => {
            debug!("Parsed event: {:?}", event);
            println!("Event: {:?}", event); // Added for debugging
            if event.event_type == *"error" || event.data.to_string().contains("error") {
                error!("Server error: {:?}", event);
                Err(SurgeError::EventError(event))
            } else if event.event_type == *"info" {
                info!("Success indicator received");
                Ok(event)
            } else {
                Ok(event)
            }
        }
        Err(FallibleNdjsonError::JsonError(e)) => {
            error!("JSON parsing error: {}", e);
            Err(SurgeError::Json(e))
        }
        Err(FallibleNdjsonError::InputError(e)) => {
            error!("Stream error: {:?}", e);
            Err(SurgeError::Io(std::io::Error::other(e.to_string())))
        }
    }))
}

/// Publishes a work-in-progress (WIP) version of a project to a preview domain.
///
/// # Arguments
/// * `client` - The `SurgeSdk` client for making HTTP requests.
/// * `project_path` - Path to the project directory.
/// * `domain` - Target domain for the preview.
/// * `auth` - Authentication credentials.
/// * `headers` - Optional custom HTTP headers.
/// * `argv` - Optional command-line arguments for the request.
///
/// # Returns
/// A `Result` containing a stream of `Event`s or a `SurgeError` if the request fails.
pub async fn publish_wip(
    client: &SurgeSdk,
    project_path: &Path,
    domain: &str,
    auth: &Auth,
    headers: Option<Vec<(String, String)>>,
    argv: Option<&[String]>,
) -> Result<impl Stream<Item = Result<Event, SurgeError>>, SurgeError> {
    info!("Publishing WIP to domain: {}", domain);
    debug!("Project path: {:?}", project_path);

    // Create a unique preview domain
    let preview_domain = format!("{}-{}", chrono::Utc::now().timestamp_millis(), domain);
    let url = format!("{}{}", client.config.endpoint, preview_domain);
    debug!("Preview URL: {}", url);

    // Calculate project metadata
    let metadata = calculate_metadata(project_path)?;
    let timestamp = chrono::Utc::now().to_rfc3339();

    // Serialize argv as JSON with stage=true
    let argv_json = argv.map_or_else(
        || {
            Ok(json!({
                "_": [],
                "e": client.config.endpoint.as_str(),
                "endpoint": client.config.endpoint.as_str(),
                "s": true,
                "stage": true
            })
            .to_string())
        },
        |args| {
            serde_json::to_string(&json!({
                "_": args,
                "e": client.config.endpoint.as_str(),
                "endpoint": client.config.endpoint.as_str(),
                "s": true,
                "stage": true
            }))
        },
    )?;

    // Build HTTP request
    let mut req = client
        .client
        .put(&url)
        .header("Content-Type", "application/gzip")
        .header("Accept", "application/ndjson")
        .header("version", &client.config.version)
        .header("timestamp", timestamp)
        .header("stage", "true")
        .header("ssl", "null")
        .header("argv", argv_json);

    req = req
        .header("file-count", metadata.file_count.to_string())
        .header("project-size", metadata.project_size.to_string());

    // Add custom headers if provided
    if let Some(headers) = headers {
        debug!("Adding custom headers: {:?}", headers);
        for (key, value) in headers {
            req = req.header(&key, value);
        }
    }

    // Create tarball stream and attach to request
    let tar_gz_stream = TarGzStream::new(project_path, 8192)?;
    req = req.body(Body::wrap_stream(tar_gz_stream));
    req = client.apply_auth(req, auth);

    // Send request
    debug!("Sending request to {}", url);
    let res = req.send().await?;
    debug!("Response status: {}", res.status());

    // Check response status
    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await?;
        error!("Request failed with status {}: {}", status, text);
        return Err(SurgeError::Api(crate::error::ApiError {
            errors: vec![format!("Request failed with status: {}", status)],
            details: Value::Object(serde_json::Map::new()),
            status: Some(status.as_u16()),
        }));
    }

    info!(
        "Successfully uploaded WIP tarball for domain: {}",
        preview_domain
    );

    // Process response as NDJSON stream
    let stream = res.bytes_stream();
    let config = NdjsonConfig::default().with_empty_line_handling(EmptyLineHandling::IgnoreEmpty);

    let stream = stream.map(|result| {
        result.map_err(SurgeError::from).and_then(|bytes| {
            trace!("Received {} bytes", bytes.len());
            String::from_utf8(bytes.to_vec()).map_err(|e| {
                error!("UTF-8 error: {}", e);
                SurgeError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            })
        })
    });

    let ndjson_stream = ndjson_stream::from_fallible_stream_with_config(stream, config);

    Ok(ndjson_stream.map(|result: Result<Event, _>| match result {
        Ok(event) => {
            debug!("Parsed event: {:?}", event);
            println!("Event: {:?}", event);
            if event.event_type == *"error" || event.data.to_string().contains("error") {
                error!("Server error: {:?}", event);
                Err(SurgeError::EventError(event))
            } else if event.event_type == *"info" {
                info!("Success indicator received");
                Ok(event)
            } else {
                Ok(event)
            }
        }
        Err(FallibleNdjsonError::JsonError(e)) => {
            error!("JSON parsing error: {}", e);
            Err(SurgeError::Json(e))
        }
        Err(FallibleNdjsonError::InputError(e)) => {
            error!("Stream error: {:?}", e);
            Err(SurgeError::Io(std::io::Error::other(e.to_string())))
        }
    }))
}

/// Builds a gitignore matcher for `.surgeignore` rules.
///
/// # Arguments
/// * `project_path` - Path to the project directory.
///
/// # Returns
/// A `Result` containing a `Gitignore` matcher or a `SurgeError` if the `.surgeignore` file is invalid.
fn build_custom_gitignore(project_path: &Path) -> Result<ignore::gitignore::Gitignore, SurgeError> {
    let mut ignore_builder = GitignoreBuilder::new(project_path);
    let surgeignore_path = project_path.join(".surgeignore");

    if surgeignore_path.exists() {
        debug!("Reading .surgeignore at: {:?}", surgeignore_path);
        for line in fs::read_to_string(&surgeignore_path)
            .map_err(|e| SurgeError::Io(std::io::Error::other(e)))?
            .lines()
        {
            ignore_builder
                .add_line(None, line)
                .map_err(SurgeError::IgnoreError)?;
        }
    } else {
        debug!(".surgeignore not found, using default ignore rules");
    }

    ignore_builder.build().map_err(SurgeError::IgnoreError)
}

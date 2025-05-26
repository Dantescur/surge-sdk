/*
  src/stream.rs
*/
use crate::{
    client::SurgeClient,
    error::SurgeError,
    types::{Auth, Event},
};
use bytes::Bytes;
use flate2::{Compression, write::GzEncoder};
use futures_util::{Stream, StreamExt};
use ignore::WalkBuilder;
use log::{debug, error, info, trace};
use ndjson_stream::{
    config::{EmptyLineHandling, NdjsonConfig},
    fallible::FallibleNdjsonError,
};
use reqwest::Body;
use serde_json::Value;
use serde_json::json;
use std::os::unix::fs::PermissionsExt;
use std::pin::Pin;
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};
use tar::{Builder, Header};
use tokio_util::bytes;

struct TarGzStream {
    project_path: PathBuf,
    walker: ignore::Walk,
    tar: Option<Builder<GzEncoder<Vec<u8>>>>,
    buffer: Vec<u8>,
    done: bool,
    dir_name: String,
    file_count: u64,
    project_size: u64,
}

impl TarGzStream {
    fn new(project_path: &Path) -> Result<Self, SurgeError> {
        debug!("Creating new TarGzStream for path: {:?}", project_path);
        if !project_path.is_dir() {
            error!("Project path {:?} is not a directory", project_path);
            return Err(SurgeError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Invalid project directory",
            )));
        }

        let dir_name = project_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("project")
            .to_string();

        let walker = WalkBuilder::new(project_path)
            .add_custom_ignore_filename(".surgeignore")
            .build();

        let encoder = GzEncoder::new(Vec::new(), Compression::new(6));
        let tar = Builder::new(encoder);

        Ok(Self {
            project_path: project_path.to_path_buf(),
            walker,
            tar: Some(tar),
            buffer: Vec::new(),
            done: false,
            dir_name,
            file_count: 0,
            project_size: 0,
        })
    }

    fn append_file(
        &mut self,
        tar: &mut Builder<GzEncoder<Vec<u8>>>,
        path: &Path,
    ) -> Result<(), SurgeError> {
        let rel_path = path
            .strip_prefix(self.project_path.parent().unwrap_or(Path::new("")))
            .map_err(|e| {
                error!("Failed to strip prefix for path {:?}: {}", path, e);
                SurgeError::Io(std::io::Error::other(e))
            })?;

        let tar_path = PathBuf::from(&self.dir_name).join(rel_path.file_name().unwrap());
        let metadata = fs::metadata(path)?;
        debug!(
            "Adding file to tar: {} (size: {}, mode: {:o})",
            tar_path.display(),
            metadata.len(),
            metadata.permissions().mode()
        );

        let mut header = Header::new_ustar();
        header.set_size(metadata.len());
        header.set_mode(0o644);
        header.set_mtime(
            metadata
                .modified()
                .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_secs())
                .unwrap_or(0),
        );
        header.set_cksum();

        debug!(
            "Tar header: name={}, mode={:o}, size={}, mtime={}",
            tar_path.display(),
            header.mode().unwrap(),
            header.size().unwrap(),
            header.mtime().unwrap(),
        );

        let mut file = File::open(path)?;
        tar.append_data(&mut header, &tar_path, &mut file)?;
        self.file_count += 1;
        self.project_size += metadata.len();
        Ok(())
    }
}

impl Stream for TarGzStream {
    type Item = Result<Bytes, SurgeError>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        trace!("Polling next chunk of TarGzStream");
        if self.done {
            debug!("TarGzStream is done, returning None");
            return std::task::Poll::Ready(None);
        }

        // Take ownership of tar builder temporarily
        let mut tar = match self.tar.take() {
            Some(tar) => tar,
            None => {
                debug!("No tar builder available, returning None");
                return std::task::Poll::Ready(None);
            }
        };

        // Process files until buffer is sufficiently full or done
        while self.buffer.len() < 8192 {
            let entry = match self.walker.next() {
                Some(Ok(entry)) => entry,
                Some(Err(e)) => {
                    error!("Error walking directory: {}", e);
                    self.tar = Some(tar); // Put tar back before returning
                    return std::task::Poll::Ready(Some(Err(SurgeError::IgnoreError(e))));
                }
                None => {
                    debug!("Finished walking directory, finalizing tar archive");
                    self.done = true;
                    break;
                }
            };

            let path = entry.path();
            trace!("Processing file: {:?}", path);
            if path.is_file() {
                if let Err(e) = self.append_file(&mut tar, path) {
                    self.tar = Some(tar); // Put tar back before returning
                    return std::task::Poll::Ready(Some(Err(e)));
                }
            }
        }

        // Put tar back before finalizing if we're not done
        if !self.done {
            self.tar = Some(tar);
            trace!("Buffer contains {} bytes", self.buffer.len());
            if self.buffer.is_empty() {
                debug!("Buffer is empty, returning None");
                return std::task::Poll::Ready(None);
            }

            let chunk_size = std::cmp::min(8192, self.buffer.len());
            let chunk = self.buffer.drain(..chunk_size).collect::<Vec<u8>>();
            debug!("Returning chunk of {} bytes", chunk.len());
            return std::task::Poll::Ready(Some(Ok(Bytes::from(chunk))));
        }

        // Finalize tarball if done
        match tar.into_inner() {
            Ok(encoder) => match encoder.finish() {
                Ok(data) => {
                    debug!("Successfully compressed tarball ({} bytes)", data.len());
                    self.buffer.extend_from_slice(&data);
                }
                Err(e) => {
                    error!("Failed to finish gzip compression: {}", e);
                    return std::task::Poll::Ready(Some(Err(SurgeError::Io(
                        std::io::Error::other(e),
                    ))));
                }
            },
            Err(e) => {
                error!("Failed to get inner encoder from tar builder: {}", e);
                return std::task::Poll::Ready(Some(Err(SurgeError::Io(std::io::Error::other(e)))));
            }
        }

        trace!("Buffer contains {} bytes", self.buffer.len());
        if self.buffer.is_empty() {
            debug!("Buffer is empty, returning None");
            return std::task::Poll::Ready(None);
        }

        let chunk_size = std::cmp::min(8192, self.buffer.len());
        let chunk = self.buffer.drain(..chunk_size).collect::<Vec<u8>>();
        debug!("Returning chunk of {} bytes", chunk.len());
        std::task::Poll::Ready(Some(Ok(Bytes::from(chunk))))
    }
}

pub async fn publish(
    client: &SurgeClient,
    project_path: &Path,
    domain: &str,
    auth: Auth,
    headers: Option<Vec<(String, String)>>,
    argv: Option<&[String]>,
) -> Result<impl Stream<Item = Result<Event, SurgeError>>, SurgeError> {
    info!("Publishing to domain: {}", domain);
    debug!("Project path: {:?}", project_path);

    let url = format!("{}/{}", client.config.endpoint, domain);
    debug!("URL: {}", url);

    let mut tar_stream = TarGzStream::new(project_path)?;
    let timestamp = chrono::Utc::now().to_rfc3339();

    let argv_json = argv.map_or_else(
        || Ok(json!({ "_": [], "e": "", "endpoint": "", "s": false, "stage": false }).to_string()),
        |args| {
            serde_json::to_string(&json!({
                "_": args,
                "e": "https://surge.surge.sh",
                "endpoint": "https://surge.surge.sh",
                "s": false,
                "stage": false
            }))
        },
    )?;

    let mut req = client
        .client
        .put(&url)
        .header("Content-Type", "")
        .header("Accept", "application/ndjson")
        .header("version", "0.24.6")
        .header("timestamp", timestamp)
        .header("stage", "false")
        .header("ssl", "null")
        .header("argv", argv_json);

    // Process the tar stream to get file_count and project_size
    let mut cx = std::task::Context::from_waker(futures::task::noop_waker_ref());
    loop {
        match Pin::new(&mut tar_stream).poll_next(&mut cx) {
            std::task::Poll::Ready(Some(Ok(_))) => continue,
            std::task::Poll::Ready(Some(Err(e))) => return Err(e),
            std::task::Poll::Ready(None) => break,
            std::task::Poll::Pending => break,
        }
    }

    req = req
        .header("file-count", tar_stream.file_count.to_string())
        .header("project-size", tar_stream.project_size.to_string());

    if let Some(headers) = headers {
        debug!("Adding custom headers: {:?}", headers);
        for (key, value) in headers {
            req = req.header(&key, value);
        }
    }

    let tar_gz_stream = TarGzStream::new(project_path)?;
    req = req.body(Body::wrap_stream(tar_gz_stream));
    req = client.apply_auth(req, auth);

    debug!("Sending request to {}", url);
    let res = req.send().await?;
    debug!("Response status: {}", res.status());

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

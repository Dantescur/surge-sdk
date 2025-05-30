// src/error.rs
use serde::Deserialize;
use thiserror::Error;

use crate::Event;

/// Unified error type for the Surge project.
///
/// Encapsulates various errors that can occur during HTTP requests, file operations,
/// URL parsing, JSON processing, and event handling.
#[derive(Error, Debug)]
pub enum SurgeError {
    /// HTTP-related errors from the `reqwest` crate.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// API errors returned by the remote server.
    #[error("API error: {0:?}")]
    Api(ApiError),

    /// Errors associated with event processing.
    #[error("Event error: {0}")]
    EventError(Event),

    /// URL parsing errors from the `url` crate.
    #[error("Parsing url error: {0}")]
    ParsingError(#[from] url::ParseError),

    /// JSON serialization/deserialization errors from `serde_json`.
    #[error("Invalid JSON: {0}")]
    Json(#[from] serde_json::Error),

    /// File system or I/O errors from the standard library.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Errors from directory traversal or ignore rules (from `ignore` crate).
    #[error("Ignore error {0}")]
    IgnoreError(#[from] ignore::Error),

    /// Catch-all for unexpected errors with a custom message.
    #[error("Unknown error occurred: {0}")]
    Other(String),
}

/// Represents an error response from the API.
///
/// Deserialized from JSON responses containing error messages, details, and an optional status code.
#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub errors: Vec<String>,
    pub details: serde_json::Value,
    pub status: Option<u16>,
}

/// Converts a `StripPrefixError` into a `SurgeError::Io`.
impl From<std::path::StripPrefixError> for SurgeError {
    fn from(e: std::path::StripPrefixError) -> Self {
        SurgeError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            e.to_string(),
        ))
    }
}

/// Converts a `tokio::task::JoinError` into a `SurgeError::Io`.
impl From<tokio::task::JoinError> for SurgeError {
    fn from(e: tokio::task::JoinError) -> Self {
        SurgeError::Io(std::io::Error::other(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io;

    /// Tests conversion of `StripPrefixError` to `SurgeError`.
    #[test]
    fn test_surge_error_from_strip_prefix() {
        let strip_err = std::path::Path::new("/a").strip_prefix("/b").unwrap_err();
        let surge_err = SurgeError::from(strip_err);
        assert!(matches!(surge_err, SurgeError::Io(_)));
        if let SurgeError::Io(e) = surge_err {
            assert_eq!(e.kind(), io::ErrorKind::InvalidData);
        }
    }

    /// Tests conversion of `JoinError` to `SurgeError`.
    #[tokio::test]
    async fn test_surge_error_from_join_error() {
        let join_err = tokio::task::spawn(async { panic!("test panic") })
            .await
            .unwrap_err();
        let surge_err = SurgeError::from(join_err);
        assert!(matches!(surge_err, SurgeError::Io(_)));
        if let SurgeError::Io(e) = surge_err {
            assert_eq!(e.kind(), io::ErrorKind::Other);
        }
    }

    /// Tests deserialization of `ApiError` from JSON.
    #[test]
    fn test_api_error_deserialization() {
        let json = json!({
            "errors": ["Invalid token"],
            "details": {},
            "status": 401
        });
        let api_err: ApiError = serde_json::from_value(json).unwrap();
        assert_eq!(api_err.errors, vec!["Invalid token"]);
        assert_eq!(api_err.status, Some(401));
        assert_eq!(api_err.details, serde_json::json!({}));
    }
}

//! Error handling module for the Surge SDK.
//!
//! This module defines the `SurgeError` enum, which provides a unified way to represent
//! all possible errors that may occur in the Surge SDK. It wraps errors from common crates
//! such as `reqwest`, `serde_json`, `url`, `ignore`, and the standard library, as well as
//! custom error types like `ApiError` and `Event`.
use serde::Deserialize;
use thiserror::Error;

use crate::Event;

/// Unified error type for the Surge SDK.
///
/// This enum encapsulates all possible error types that might occur during SDK usage, including:
/// - HTTP request failures
/// - Server-side API errors
/// - Event processing errors
/// - URL parsing issues
/// - JSON (de)serialization issues
/// - File system or I/O errors
/// - Ignore rules and directory walking issues
/// - Other unexpected or miscellaneous errors
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
/// This struct is used to deserialize error responses returned by the remote server.
/// It typically includes a list of error messages, optional HTTP status code, and any
/// additional details provided in the response body.
#[derive(Debug, Deserialize)]
pub struct ApiError {
    /// A list of error messages provided by the API.
    pub errors: Vec<String>,

    /// A JSON value with additional details (may be an object, array, or primitive).
    pub details: serde_json::Value,

    /// Optional HTTP status code.
    pub status: Option<u16>,
}

/// Converts a `StripPrefixError` into a `SurgeError::Io`.
///
/// This is useful when paths cannot be relativized, and we want to represent this
/// failure as a standard I/O error.
impl From<std::path::StripPrefixError> for SurgeError {
    fn from(e: std::path::StripPrefixError) -> Self {
        SurgeError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            e.to_string(),
        ))
    }
}

/// Converts a `tokio::task::JoinError` into a `SurgeError::Io`.
///
/// This enables seamless propagation of async task join errors within the SDK.
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

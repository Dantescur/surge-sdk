/*
  src/error.rs
*/
//! Error handling module for the Surge SDK.
//!
//! This module defines the `SurgeError` enum, which provides a unified way to represent
//! all possible errors that may occur in the Surge SDK. It wraps errors from common crates
//! such as `reqwest`, `serde_json`, `url`, `ignore`, and the standard library, as well as
//! custom error types like `ApiError` and `Event`.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

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
#[derive(Error, Debug, Deserialize, Serialize)]
pub enum SurgeError {
    /// HTTP-related errors from the `reqwest` crate.
    #[error("HTTP error: {0}")]
    Http(String),

    /// API errors returned by the remote server.
    #[error("API error (status: {status:?}): {message}")]
    Api {
        status: Option<u16>,
        message: String,
        details: Value,
    },

    /// TLS errors
    #[error("TLS error: {0}")]
    Tls(String),

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(String),

    /// File system or I/O errors
    #[error("IO error: {0}")]
    Io(String),

    /// Directory traversal or ignore rules errors
    #[error("Ignore error: {0}")]
    Ignore(String),

    /// Invalid project directory structure
    #[error("Invalid project: {0}")]
    InvalidProject(String),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Network errors
    #[error("Network error: {0}")]
    Network(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Event processing errors
    #[error("Event error: {0}")]
    Event(String),

    /// Unknown error variant
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl SurgeError {
    /// Creates a new API error
    pub fn api(status: Option<u16>, message: impl Into<String>, details: Value) -> Self {
        SurgeError::Api {
            status,
            message: message.into(),
            details,
        }
    }
}

// Implement From traits for common error types
impl From<reqwest::Error> for SurgeError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_status() {
            SurgeError::Http(format!("HTTP status error: {}", err))
        } else if err.is_timeout() {
            SurgeError::Network(format!("Request timeout: {}", err))
        } else if err.is_connect() {
            SurgeError::Network(format!("Connection error: {}", err))
        } else {
            SurgeError::Http(format!("HTTP error: {}", err))
        }
    }
}

impl From<std::io::Error> for SurgeError {
    fn from(err: std::io::Error) -> Self {
        SurgeError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for SurgeError {
    fn from(err: serde_json::Error) -> Self {
        SurgeError::Json(err.to_string())
    }
}

impl From<ignore::Error> for SurgeError {
    fn from(err: ignore::Error) -> Self {
        SurgeError::Ignore(err.to_string())
    }
}

impl From<url::ParseError> for SurgeError {
    fn from(err: url::ParseError) -> Self {
        SurgeError::Config(format!("URL parse error: {}", err))
    }
}

impl From<rustls::Error> for SurgeError {
    fn from(err: rustls::Error) -> Self {
        SurgeError::Tls(err.to_string())
    }
}

impl From<std::path::StripPrefixError> for SurgeError {
    fn from(err: std::path::StripPrefixError) -> Self {
        SurgeError::InvalidProject(err.to_string())
    }
}

impl From<tokio::task::JoinError> for SurgeError {
    fn from(err: tokio::task::JoinError) -> Self {
        SurgeError::Unknown(err.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for SurgeError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        SurgeError::Unknown(err.to_string())
    }
}

/// Simplified API error response
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiErrorResponse {
    pub errors: Vec<String>,
    pub details: Value,
    pub status: Option<u16>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Tests conversion of `StripPrefixError` to `SurgeError`.
    #[test]
    fn test_surge_error_from_strip_prefix() {
        let strip_err = std::path::Path::new("/a").strip_prefix("/b").unwrap_err();
        let surge_err = SurgeError::from(strip_err);
        assert!(matches!(surge_err, SurgeError::InvalidProject(_))); // Fixed: Matches InvalidProject
        if let SurgeError::InvalidProject(msg) = surge_err {
            assert!(msg.contains("strip prefix"));
        }
    }

    /// Tests conversion of `JoinError` to `SurgeError`.
    #[tokio::test]
    async fn test_surge_error_from_join_error() {
        let join_err = tokio::task::spawn(async { panic!("test panic") })
            .await
            .unwrap_err();
        let surge_err = SurgeError::from(join_err);
        assert!(matches!(surge_err, SurgeError::Unknown(_))); // Fixed: Matches Unknown
        if let SurgeError::Unknown(msg) = surge_err {
            assert!(msg.contains("test panic"));
        }
    }

    /// Tests deserialization of `ApiErrorResponse`:

    #[test]
    fn test_api_error_deserialization() {
        let json = json!({
            "errors": ["Invalid token"],
            "details": {},
            "status": 401
        });
        let api_err: ApiErrorResponse = serde_json::from_value(json).unwrap();
        assert_eq!(api_err.errors, vec!["Invalid token"]);
        assert_eq!(api_err.status, Some(401));
        assert_eq!(api_err.details, serde_json::json!({}));
    }
}

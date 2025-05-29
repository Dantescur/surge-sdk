// src/error.rs
use serde::Deserialize;
use thiserror::Error;

use crate::Event;

#[derive(Error, Debug)]
pub enum SurgeError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error: {0:?}")]
    Api(ApiError),
    #[error("Event error: {0}")]
    EventError(Event),
    #[error("Parsing url error: {0}")]
    ParsingError(#[from] url::ParseError),
    #[error("Invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Ignore error {0}")]
    IgnoreError(#[from] ignore::Error),
    #[error("Unknown error occurred: {0}")]
    Other(String),
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub errors: Vec<String>,
    pub details: serde_json::Value,
    pub status: Option<u16>,
}

impl From<std::path::StripPrefixError> for SurgeError {
    fn from(e: std::path::StripPrefixError) -> Self {
        SurgeError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            e.to_string(),
        ))
    }
}
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

    #[test]
    fn test_surge_error_from_strip_prefix() {
        let strip_err = std::path::Path::new("/a").strip_prefix("/b").unwrap_err();
        let surge_err = SurgeError::from(strip_err);
        assert!(matches!(surge_err, SurgeError::Io(_)));
        if let SurgeError::Io(e) = surge_err {
            assert_eq!(e.kind(), io::ErrorKind::InvalidData);
        }
    }

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

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

// src/types.rs
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Auth {
    Token(String),
    UserPass { username: String, password: String },
}

// Event type for publish/encrypt streams
#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(flatten)]
    pub data: serde_json::Value,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Event: {}] {}",
            self.event_type,
            serde_json::to_string_pretty(&self.data).unwrap_or_else(|_| "<invalid JSON>".into())
        )
    }
}

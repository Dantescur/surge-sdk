// src/types.rs
use serde::Deserialize;
use std::fmt;

/// Authentication credentials for API requests.
///
/// Supports token-based or username/password authentication.
#[derive(Debug, Clone)]
pub enum Auth {
    /// Token-based authentication with a single token string.
    Token(String),
    /// Username and password authentication.
    UserPass { username: String, password: String },
}

/// An event from NDJSON streaming endpoints (e.g., publish, encrypt).
///
/// Contains an event type and arbitrary JSON data.
#[derive(Debug, Deserialize)]
pub struct Event {
    /// The type of event (e.g., "info", "error").
    #[serde(rename = "type")]
    pub event_type: String,
    /// Additional event data as a JSON value.
    #[serde(flatten)]
    pub data: serde_json::Value,
}

impl fmt::Display for Event {
    /// Formats the event as a string for display.
    ///
    /// # Example
    /// ```
    /// use surge_sdk::Event;
    ///
    /// let event = Event {
    ///     event_type: "info".to_string(),
    ///     data: serde_json::json!({ "message": "Success" }),
    /// };
    /// println!("{}", event); // Outputs: [Event: info] { "message": "Success" }
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Event: {}] {}",
            self.event_type,
            serde_json::to_string_pretty(&self.data).unwrap_or_else(|_| "<invalid JSON>".into())
        )
    }
}

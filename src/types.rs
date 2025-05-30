//! Module defining core data types for the Surge SDK.
//!
//! This module provides essential data structures used throughout the Surge SDK for interacting
//! with the Surge API. It includes types for authentication credentials and events received from
//! streaming endpoints. These types are designed to be lightweight, serializable, and easy to use
//! in conjunction with the SDK's asynchronous operations.
//!
//! Key types include:
//! - `Auth`: An enum supporting token-based or username/password authentication for API requests.
//! - `Event`: A struct representing events from NDJSON streaming endpoints, such as those used in
//!   publishing or encryption operations, with a type identifier and arbitrary JSON data.
//!
//! The types are implemented with `serde` for deserialization and include display formatting for
//! easier debugging and logging. The module is intended to be used by other parts of the SDK,
//! such as the `SurgeSdk` client and streaming utilities, to handle authentication and process
//! API responses.
//!
//! # Example
//! ```
//! use surge_sdk::types::{Auth, Event};
//! use serde_json::json;
//!
//! // Example of creating authentication credentials
//! let auth = Auth::Token("your-api-token".to_string());
//!
//! // Example of creating an event
//! let event = Event {
//!     event_type: "info".to_string(),
//!     data: json!({ "message": "Operation successful" }),
//! };
//! println!("{}", event); // Outputs: [Event: info] { "message": "Operation successful" }
//! ```

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
    UserPass {
        /// Username (email)
        username: String,
        /// Password (token)
        password: String,
    },
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

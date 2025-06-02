/*
  src/types.rs
*/
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

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
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

// FIX: Change comments lang in the future
/// Evento deserializado en bruto, usado como paso previo para mapear al enum `Event`.
///
/// Este struct conserva el campo `type` como string para poder discriminar entre eventos conocidos
/// y permite conservar el resto de los datos sin pérdida gracias a `#[serde(flatten)]`.
#[derive(Debug, Deserialize)]
pub struct RawEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CertEventData {
    pub issuer: String,
    #[serde(rename = "altnames")]
    pub alt_names: Vec<String>,
    #[serde(rename = "expiresInWords")]
    pub expires_in_words: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CertDetails {
    pub subject: String,
    pub issuer: String,
    #[serde(rename = "notBefore")]
    pub not_before: String,
    #[serde(rename = "notAfter")]
    pub not_after: String,
    #[serde(rename = "expInDays")]
    pub exp_in_days: u32,
    #[serde(rename = "subjectAltNames")]
    pub subject_alt_names: Vec<String>,
    #[serde(rename = "certName")]
    pub cert_name: String,
    #[serde(rename = "autoRenew")]
    pub auto_renew: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Instance {
    pub confirmation: String,
    #[serde(rename = "confirmationColor")]
    pub confirmation_color: String,
    pub domain: String,
    pub info: String,
    pub ip: String,
    pub location: String,
    pub provider: Option<String>,
    pub status: String,
    #[serde(rename = "statusColor")]
    pub status_color: String,
    #[serde(rename = "type")]
    pub instance_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Url {
    pub domain: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub cors: Option<Value>,
    pub force: Option<Value>,
    pub hsts: Option<Value>,
    pub redirect: Option<Value>,
    pub ttl: Option<Value>,
    #[serde(default)]
    pub pdf: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    #[serde(rename = "buildTime")]
    pub build_time: Option<String>,
    #[serde(rename = "cliVersion")]
    pub cli_version: String,
    pub cmd: String,
    pub config: Config,
    pub current: bool,
    pub email: String,
    pub ip: String,
    pub message: Option<String>,
    pub output: Value,
    pub platform: String,
    pub preview: String,
    #[serde(rename = "privateFileCount")]
    pub private_file_count: u64,
    #[serde(rename = "privateFileList")]
    pub private_file_list: Vec<String>,
    #[serde(rename = "publicFileCount")]
    pub public_file_count: u64,
    #[serde(rename = "publicTotalSize")]
    pub public_total_size: u64,
    pub rev: u64,
    #[serde(rename = "uploadDuration")]
    pub upload_duration: f64,
    #[serde(rename = "uploadEndTime")]
    pub upload_end_time: u64,
    #[serde(rename = "uploadStartTime")]
    pub upload_start_time: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InfoEventData {
    pub certs: Vec<CertDetails>,
    pub config: Config,
    pub instances: Vec<Instance>,
    pub metadata: Metadata,
    pub urls: Vec<Url>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IpEventData {
    pub ip: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriptionEventData {
    #[serde(default)]
    pub data: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub enum Event {
    Cert(CertEventData),
    Progress {
        id: String,
        written: u64,
        total: u64,
        end: Option<bool>,
    },
    Info(InfoEventData),
    Ip(IpEventData),
    Subscription(SubscriptionEventData),
    Unknown {
        event_type: String,
        data: Value,
    },
}

fn deserialize_written<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(u64),
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => s.parse().map_err(serde::de::Error::custom),
        StringOrNumber::Number(n) => Ok(n),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ProgressData {
    id: String,
    #[serde(deserialize_with = "deserialize_written")]
    written: u64,
    total: u64,
    end: Option<bool>,
}

impl From<RawEvent> for Event {
    fn from(raw: RawEvent) -> Self {
        match raw.event_type.as_str() {
            "cert" => {
                let parsed = serde_json::from_value::<Value>(raw.data.clone())
                    .and_then(|v| serde_json::from_value::<CertEventData>(v["data"].clone()));
                match parsed {
                    Ok(data) => Event::Cert(data),
                    Err(_) => Event::Unknown {
                        event_type: raw.event_type,
                        data: raw.data,
                    },
                }
            }
            "progress" => {
                let parsed = serde_json::from_value::<ProgressData>(raw.data.clone());
                match parsed {
                    Ok(p) => Event::Progress {
                        id: p.id,
                        written: p.written,
                        total: p.total,
                        end: p.end,
                    },
                    Err(_) => Event::Unknown {
                        event_type: raw.event_type,
                        data: raw.data,
                    },
                }
            }
            "info" => {
                let parsed = serde_json::from_value::<InfoEventData>(raw.data.clone());
                match parsed {
                    Ok(data) => Event::Info(data),
                    Err(_) => Event::Unknown {
                        event_type: raw.event_type,
                        data: raw.data,
                    },
                }
            }
            "ip" => {
                let parsed = serde_json::from_value::<Value>(raw.data.clone())
                    .and_then(|v| serde_json::from_value::<IpEventData>(v["data"].clone()));
                match parsed {
                    Ok(data) => Event::Ip(data),
                    Err(_) => Event::Unknown {
                        event_type: raw.event_type,
                        data: raw.data,
                    },
                }
            }
            "subscription" => {
                let parsed = serde_json::from_value::<Value>(raw.data.clone()).and_then(|v| {
                    serde_json::from_value::<SubscriptionEventData>(v["data"].clone())
                });
                match parsed {
                    Ok(data) => Event::Subscription(data),
                    Err(_) => Event::Unknown {
                        event_type: raw.event_type,
                        data: raw.data,
                    },
                }
            }
            _ => Event::Unknown {
                event_type: raw.event_type,
                data: raw.data,
            },
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::Cert(data) => write!(
                f,
                "[Event: cert] Issuer: {}, Alt Names: {:?}, Expires: {}",
                data.issuer, data.alt_names, data.expires_in_words
            ),
            Event::Progress {
                id,
                written,
                total,
                end,
            } => {
                let percentage = if *total > 0 {
                    (*written as f64 / *total as f64 * 100.0).round() as u64
                } else {
                    0
                };
                write!(
                    f,
                    "[Event: progress] ID: {}, Progress: {}/{} ({}%), Complete: {}",
                    id,
                    written,
                    total,
                    percentage,
                    end.unwrap_or(false)
                )
            }
            Event::Info(data) => {
                let cert_summary = if data.certs.is_empty() {
                    "No certificates".to_string()
                } else {
                    format!("{} certificate(s)", data.certs.len())
                };
                let instance_summary = format!("{} instance(s)", data.instances.len());
                let urls_summary = data
                    .urls
                    .iter()
                    .map(|u| u.domain.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(
                    f,
                    "[Event: info] Certs: {}, Instances: {}, URLs: [{}], Platform: {}, Email: {}",
                    cert_summary,
                    instance_summary,
                    urls_summary,
                    data.metadata.platform,
                    data.metadata.email
                )
            }
            Event::Ip(data) => write!(f, "[Event: ip] IP: {}", data.ip),
            Event::Subscription(_) => write!(f, "[Event: subscription] Subscription event"),
            Event::Unknown { event_type, data } => write!(
                f,
                "[Event: {}] {}",
                event_type,
                serde_json::to_string_pretty(data).unwrap_or_else(|_| "<invalid JSON>".into())
            ),
        }
    }
}

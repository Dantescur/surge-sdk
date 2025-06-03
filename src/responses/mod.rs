/*
  src/responses/mod.rs
*/
//! # Response Types for Surge API
//!
//! This module contains all structured response types returned by the Surge API.
//! Each submodule maps to a different endpoint or logical feature, and exports
//! strongly-typed structures to deserialize HTTP responses. These types are used
//! across the application to ensure safe and predictable handling of API data.
mod account;
mod certs;
mod danalytics;
mod daudit;
mod discard;
mod list;
mod login;
mod manifest;
mod metadata;
mod plans;
mod roll;
mod settings;
mod stripe;
mod teardown;
mod uploadfin;
mod usage;

/// Re-exports the unified error type for response handling.
pub use crate::error::SurgeError;

/// Represents the authenticated user's account information.
pub use account::AccountResponse;

/// Represents a response containing usage statistics
pub use usage::UsageResponse;

/// Represents a response containing deployment certificates.
pub use certs::{Cert as Certs, CertsResponse};

/// Represents analytics data about deployments or traffic.
pub use danalytics::DAnalyticsResponse;

/// Represents settings status
pub use settings::SettingsResponse;

/// Represents deployment audit logs or changes.
pub use daudit::DAuditResponse;

/// Represents the list of deployments, including associated plans.
pub use list::{ListDomainResponse, ListResponse, ListResult};

/// Represents a discard response result
pub use discard::DiscardResponse;

/// Represents the result of a login operation, typically containing tokens or session info.
pub use login::LoginResponse;

/// Represents the result of rolling back a rev
pub use roll::RollResponse;

/// Represents the deployment manifest returned after a successful upload or update.
pub use manifest::ManifestResponse;

/// Represents all available plans a user can subscribe to.
pub use plans::PlansResponse;

/// Represents the finalization state of an upload process.
pub use uploadfin::UploadFinResponse;

/// Represents a teardown response
pub use teardown::TeardownResponse;

/// Represents the result of an metadata response.
pub use metadata::MetadataResponse;

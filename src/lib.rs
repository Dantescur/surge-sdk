//! # Surge SDK for Rust
//!
//! A type-safe Rust interface for the [Surge.sh](https://surge.sh) API, enabling programmatic
//! management of static site deployments, domains, SSL certificates, and DNS records.
//!
//! ## Key Features
//! - 🚀 Zero-config publishing to `.surge.sh` domains
//! - 🔒 SSL certificate management (requires Pro account)
//! - 🌐 DNS and domain zone configuration
//! - 📊 Real-time deployment event streaming
//! - 🛠️ Async-first design using `reqwest` and `tokio`
//!
//! ## Quick Start
//! ```rust,no_run
//! use surge_sdk::{Config, SurgeSdk, Auth, SURGE_API};
//! use std::path::Path;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), surge_sdk::SurgeError> {
//!     let config = Config::new(SURGE_API, "0.1.0")?;
//!     let sdk = SurgeSdk::new(config)?;
//!     let auth = Auth::Token("your-api-token".into());
//!     
//!     sdk.publish(Path::new("./dist"), "your-domain.surge.sh", &auth, None, None).await?;
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod error;
pub mod responses;
pub mod sdk;
pub mod stream;
pub mod types;
pub mod utils;

pub use config::Config;
pub use error::SurgeError;
pub use responses::*;
pub use sdk::SurgeSdk;
pub use stream::{calculate_metadata, publish, publish_wip};
pub use types::{Auth, Event};
pub use utils::{generate_domain, json_to_argv};

/// The default Surge.sh API endpoint
///
/// Used as the default base URL in [`Config`]:
/// ```rust
/// use surge_sdk::{Config, SURGE_API};
///
/// // These are equivalent:
/// let cfg1 = Config::new(SURGE_API, "0.1.0");
/// let cfg2 = Config::new("https://surge.surge.sh", "0.1.0");
/// ```
pub const SURGE_API: &str = "https://surge.surge.sh";


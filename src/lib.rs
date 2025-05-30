/*
  lib.rs
*/
mod config;
mod error;
mod responses;
mod sdk;
mod stream;
mod types;
mod utils;

pub use config::Config;
pub use error::SurgeError;
pub use responses::*;
pub use sdk::SurgeSdk;
pub use stream::{calculate_metadata, publish, publish_wip};
pub use types::{Auth, Event};
pub use utils::{generate_domain, json_to_argv};

// Utility constant, since is unlickely that the endpoint change
pub const SURGE_API: &str = "https://surge.surge.sh";

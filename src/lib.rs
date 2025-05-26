/*
  src/lib.rs
*/
/*
  lib.rs
*/
mod client;
mod config;
mod error;
mod responses;
mod sdk;
mod stream;
mod types;
mod ui;

pub use client::SurgeClient;
pub use config::Config;
pub use error::SurgeError;
pub use responses::*;
pub use types::{Auth, Event};
pub use ui::print_domain_list;

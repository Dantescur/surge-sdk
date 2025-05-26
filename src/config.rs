// src/config.rs
pub struct Config {
    pub endpoint: String,
    pub version: String,
}

impl Config {
    pub fn new(endpoint: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            version: version.into(),
        }
    }
}

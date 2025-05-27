use url::Url;

// src/config.rs
pub struct Config {
    pub endpoint: Url,
    pub version: String,
    pub insecure: bool,
    pub timeout_secs: u64,
}

impl Config {
    pub fn new(
        endpoint: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Self, url::ParseError> {
        Ok(Self {
            endpoint: Url::parse(&endpoint.into())?,
            version: version.into(),
            timeout_secs: 30,
            insecure: false,
        })
    }
}

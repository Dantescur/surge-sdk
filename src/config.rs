use url::Url;

// src/config.rs
#[derive(Debug)]
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

    pub fn with_insecure(mut self, val: bool) -> Self {
        self.insecure = val;
        self
    }

    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }
}

#[cfg(test)]
mod test {
    use url::Url;

    use super::Config;

    #[test]
    fn test_config_new_valid_url() {
        let config = Config::new("https://example.com", "0.1.0").unwrap();
        assert_eq!(config.endpoint, Url::parse("https://example.com").unwrap());
        assert_eq!(config.version, "0.1.0");
        assert_eq!(config.timeout_secs, 30);
        assert!(!config.insecure);
    }

    #[test]
    fn test_config_new_invalid_url() {
        let result = Config::new("invalid-url", "0.1.0");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            url::ParseError::RelativeUrlWithoutBase
        ));
    }
}

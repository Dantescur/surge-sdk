/*
  src/config.rs
*/
//! Configuration module for the Surge SDK.
//!
//! Provides the `Config` struct used to configure SDK behavior including:
//! - API endpoint URL
//! - Client version
//! - Connection timeout
//! - Security settings
//!
//! This module ensures that configuration is easy to construct, validate, and extend with
//! builder-style methods for convenience.

use url::Url;

/// Configuration settings for the SDK.
///
/// Holds the API endpoint, version, timeout duration, and security settings.
///
/// # Fields
/// - `endpoint`: Base URL for the Surge API (must be a valid `Url`)
/// - `version`: SDK or client version string
/// - `insecure`: Whether to allow insecure HTTP connections (default is `false`)
/// - `timeout_secs`: Timeout in seconds for network operations (default is `30`)
#[derive(Debug)]
pub struct Config {
    /// The base API endpoint URL.
    pub endpoint: Url,

    /// The client or SDK version string.
    pub version: String,

    /// Whether to allow insecure HTTP connections (e.g., without TLS).
    pub insecure: bool,

    /// Timeout duration for API calls, in seconds.
    pub timeout_secs: u64,
}

impl Config {
    /// Creates a new `Config` with default timeout and secure settings.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint URL (must be a valid URL).
    /// * `version` - The version of the client or protocol.
    ///
    /// # Returns
    /// A `Result` containing the `Config` or a `url::ParseError` if the endpoint is invalid.
    ///
    /// # Example
    /// ```
    ///use surge_sdk::{Config,SURGE_API};
    ///
    /// let config = Config::new(SURGE_API, "0.1.0").unwrap();
    /// assert_eq!(config.endpoint.as_str(), "https://surge.surge.sh/");
    /// assert_eq!(config.version, "0.1.0");
    /// ```
    pub fn new(
        endpoint: impl Into<String>, // Accepts any type that can be converted to String
        version: impl Into<String>,  // Accepts any type that can be converted to String
    ) -> Result<Self, url::ParseError> {
        Ok(Self {
            endpoint: Url::parse(&endpoint.into())?,
            version: version.into(),
            timeout_secs: 30,
            insecure: false,
        })
    }

    /// Sets the `insecure` flag to allow or disallow insecure connections.
    ///
    /// **Warning**: Enabling `insecure` (setting to `true`) disables TLS verification,
    /// which can expose your application to man-in-the-middle attacks. Use only for testing.
    ///
    /// # Arguments
    /// * `val` - Whether to enable insecure connections.
    ///
    /// # Returns
    /// The modified `Config` instance for method chaining.
    ///
    /// # Example
    /// ```
    /// use surge_sdk::{Config, SURGE_API};
    ///
    /// let config = Config::new(SURGE_API, "0.1.0")
    ///     .unwrap()
    ///     .with_insecure(true);
    /// assert!(config.insecure);
    /// ```
    pub fn with_insecure(mut self, val: bool) -> Self {
        self.insecure = val;
        self
    }

    /// Sets the timeout duration in seconds.
    ///
    /// # Arguments
    /// * `secs` - Timeout duration in seconds.
    ///
    /// # Returns
    /// The modified `Config` instance for method chaining.
    ///
    /// # Example
    /// ```
    /// use surge_sdk::{Config, SURGE_API};
    ///
    /// let config = Config::new(SURGE_API, "0.1.0")
    ///     .unwrap()
    ///     .with_timeout(60);
    /// assert_eq!(config.timeout_secs, 60);
    /// ```
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }
}

#[cfg(test)]
mod test {
    use url::Url;

    use crate::SURGE_API;

    use super::Config;

    /// Tests creating a `Config` with a valid URL.
    #[test]
    fn test_config_new_valid_url() {
        let config = Config::new(SURGE_API, "0.1.0").unwrap();
        assert_eq!(
            config.endpoint,
            Url::parse("https://surge.surge.sh").unwrap()
        );
        assert_eq!(config.version, "0.1.0");
        assert_eq!(config.timeout_secs, 30);
        assert!(!config.insecure);
    }

    /// Tests that an invalid URL results in a parsing error.
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

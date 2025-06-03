/*
  src/sdk.rs
*/
//! Surge SDK for interacting with the Surge API.
//!
//! This module provides a comprehensive SDK for managing domains, publishing projects, and handling
//! account operations with the Surge API. It encapsulates an HTTP client configured with user-specified
//! settings and provides methods for various API operations such as account management, domain
//! operations, DNS configuration, SSL certificate management, and more. The SDK supports both token-based
//! and username/password authentication, and it handles streaming responses for operations like project
//! publishing and SSL encryption.
//!
//! The main entry point is the `SurgeSdk` struct, which holds the configuration and HTTP client. All API
//! interactions are performed asynchronously using the `reqwest` crate, and responses are deserialized
//! into appropriate Rust types or raw JSON values where applicable. Errors are handled using the
//! `SurgeError` type, which encapsulates various failure modes such as HTTP errors, JSON parsing errors,
//! and API-specific errors.
//!
//! # Features
//! - Account management: Fetch account details, update plans, and manage payment methods.
//! - Domain operations: List, publish, rollback, and tear down domains.
//! - DNS and SSL: Manage DNS records, SSL certificates, and encryption requests.
//! - Streaming support: Handle streaming responses for publishing and encryption operations.
//! - Authentication: Supports both token-based and username/password authentication.
//!
//! # Example
//! ```rust,no_run
//! use surge_sdk::{Config, SurgeSdk, Auth, SURGE_API};
//! # async fn example() -> Result<(), surge_sdk::error::SurgeError> {
//! let config = Config::new(SURGE_API, "0.1.0").unwrap();
//! let sdk = SurgeSdk::new(config)?;
//! let auth = Auth::Token("your-api-token".to_string());
//! let account = sdk.account(&auth).await?;
//! println!("Account: {:?}", account);
//! # Ok(())
//! # }
//! ```
use futures_util::Stream;
use log::debug;
use rustls::{ClientConfig, RootCertStore};
use serde_json::Value;
use std::{fs, path::Path, time::Duration};

use reqwest::Client;

use crate::{
    CertsResponse, DAnalyticsResponse, DAuditResponse, DiscardResponse, ListDomainResponse,
    ListResponse, ListResult, ManifestResponse, MetadataResponse, PlansResponse, RollResponse,
    TeardownResponse,
    config::Config,
    error::{ApiErrorResponse, SurgeError},
    responses::{AccountResponse, LoginResponse},
    types::{Auth, Event},
};

/// SDK for interacting with the Surge API.
///
/// Encapsulates an HTTP client and configuration for managing domains, publishing projects,
/// and handling account operations.
pub struct SurgeSdk {
    /// Configuration settings for the SDK, including the API endpoint and timeout settings.
    pub config: Config,
    /// The HTTP client used for making API requests, configured with the provided settings.
    pub client: Client,
}

impl SurgeSdk {
    /// Creates a new `SurgeSdk` instance with the given configuration.
    ///
    /// # Arguments
    /// * `config` - Configuration settings for the SDK.
    ///
    /// # Returns
    /// A `Result` containing the `SurgeSdk` or a `SurgeError` if the HTTP client cannot be built.
    ///
    /// # Example
    /// ```
    /// use surge_sdk::{Config,SurgeSdk};
    /// let config = Config::new("https://api.surge.sh", "0.1.0").unwrap();
    /// let sdk = SurgeSdk::new(config).unwrap();
    /// ```
    pub fn new(config: Config) -> Result<Self, SurgeError> {
        let client = if cfg!(feature = "rustls") {
            rustls::crypto::ring::default_provider()
                .install_default()
                .map_err(|e| SurgeError::Http(format!("Failed to set crypto provider: {:?}", e)))?;

            let mut root_store = RootCertStore::empty();
            root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

            let tls_confg = ClientConfig::builder()
                .with_root_certificates(root_store)
                .with_no_client_auth();

            Client::builder()
                .timeout(Duration::from_secs(config.timeout_secs))
                .danger_accept_invalid_certs(config.insecure)
                .use_preconfigured_tls(tls_confg)
                .build()
                .map_err(|e| SurgeError::Http(e.to_string()))?
        } else {
            Client::builder()
                .timeout(Duration::from_secs(config.timeout_secs))
                .danger_accept_invalid_certs(config.insecure)
                .build()
                .map_err(|e| SurgeError::Http(e.to_string()))?
        };

        Ok(Self { config, client })
    }

    /// Fetches account information.
    ///
    /// # Arguments
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing an `AccountResponse` or a `SurgeError`.
    pub async fn account(&self, auth: &Auth) -> Result<AccountResponse, SurgeError> {
        let url = self.config.endpoint.join("account")?;
        let req = self.apply_auth(self.client.get(url), auth);
        debug!("Request sended to account: {:#?}", req);
        let res = req.send().await?.json().await?;
        debug!("Response received: {:#?}", res);
        Ok(res)
    }

    /// Lists domains, optionally filtered by a specific domain.
    ///
    /// # Arguments
    /// * `domain` - Optional domain to filter the list.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing a `ListResponse` or a `SurgeError`.
    pub async fn list(&self, domain: Option<&str>, auth: &Auth) -> Result<ListResult, SurgeError> {
        let path = match domain {
            Some(d) => format!("{}/list", d),
            None => "list".to_string(),
        };
        let url = self.config.endpoint.join(&path)?;
        let req = self.apply_auth(self.client.get(url), auth);
        debug!("Request sent to list: {:#?}", req);

        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);

        match domain {
            Some(_) => {
                let domain_response: ListDomainResponse = serde_json::from_str(&body_text)?;
                Ok(ListResult::Domain(domain_response))
            }
            None => {
                let global_response: Vec<ListResponse> = serde_json::from_str(&body_text)?;
                Ok(ListResult::Global(global_response))
            }
        }
    }

    /// Deletes the account.
    ///
    /// # Arguments
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` indicating success or a `SurgeError`.
    pub async fn nuke(&self, auth: &Auth) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join("account")?;
        let req = self.apply_auth(self.client.delete(url), auth);
        debug!("Request sent to nuke: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        Ok(())
    }

    /// Tears down a domain.
    ///
    /// # Arguments
    /// * `domain` - The domain to tear down.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing `TeardownResponse` if the operation was successful, or a `SurgeError`.
    pub async fn teardown(
        &self,
        domain: &str,
        auth: &Auth,
    ) -> Result<TeardownResponse, SurgeError> {
        let url = self.config.endpoint.join(domain)?;
        let req = self.apply_auth(self.client.delete(url), auth);
        debug!("Request sent to teardown: {:#?}", &req);
        let response = req.send().await?;
        let body_text = response.text().await?;
        debug!("response raw: {:?}", body_text);

        let teardown_response: TeardownResponse = serde_json::from_str(&body_text)?;
        Ok(teardown_response)
    }

    /// Logs in to the API.
    ///
    /// # Arguments
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing a `LoginResponse` or a `SurgeError`.
    pub async fn login(&self, auth: &Auth) -> Result<LoginResponse, SurgeError> {
        let url = self.config.endpoint.join("token")?;
        let req = self.apply_auth(self.client.post(url), auth);
        debug!("Request sent to login: {:#?}", req);
        let res = req.send().await?;
        let status = res.status();
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);

        if status.is_success() {
            let login_response: LoginResponse = serde_json::from_str(&body_text)?;
            Ok(login_response)
        } else {
            // Try to deserialize the error response
            match serde_json::from_str::<ApiErrorResponse>(&body_text) {
                Ok(api_error) => Err(SurgeError::Api {
                    status: api_error.status,
                    message: api_error.errors.join("; "),
                    details: api_error.details,
                }),
                Err(_) => Err(SurgeError::Http(format!(
                    "HTTP error: status {}, body: {}",
                    status, body_text
                ))),
            }
        }
    }

    /// Publishes a project directory to a domain.
    ///
    /// Delegates to `stream::publish` for tarball creation and streaming.
    ///
    /// # Arguments
    /// * `project_path` - Path to the project directory.
    /// * `domain` - Target domain for publishing.
    /// * `auth` - Authentication credentials.
    /// * `headers` - Optional custom HTTP headers.
    /// * `argv` - Optional command-line arguments.
    ///
    /// # Returns
    /// A `Result` containing a stream of `Event`s or a `SurgeError`.
    pub async fn publish(
        &self,
        project_path: &Path,
        domain: &str,
        auth: &Auth,
        headers: Option<Vec<(String, String)>>,
        argv: Option<&[String]>,
    ) -> Result<impl Stream<Item = Result<Event, SurgeError>>, SurgeError> {
        crate::stream::publish(self, project_path, domain, auth, headers, argv).await
    }

    /// Publishes a work-in-progress version of a project to a preview domain.
    ///
    /// Delegates to `stream::publish_wip` for tarball creation and streaming.
    ///
    /// # Arguments
    /// * `project_path` - Path to the project directory.
    /// * `domain` - Target domain for the preview.
    /// * `auth` - Authentication credentials.
    /// * `headers` - Optional custom HTTP headers.
    /// * `argv` - Optional command-line arguments.
    ///
    /// # Returns
    /// A `Result` containing a stream of `Event`s or a `SurgeError`.
    pub async fn publish_wip(
        &self,
        project_path: &Path,
        domain: &str,
        auth: &Auth,
        headers: Option<Vec<(String, String)>>,
        argv: Option<&[String]>,
    ) -> Result<impl Stream<Item = Result<Event, SurgeError>>, SurgeError> {
        crate::stream::publish_wip(self, project_path, domain, auth, headers, argv).await
    }

    /// Rolls back a domain to a previous revision.
    ///
    /// # Arguments
    /// * `domain` - The domain to roll back.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` indicating success or a `SurgeError`.
    pub async fn rollback(&self, domain: &str, auth: &Auth) -> Result<RollResponse, SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/rollback", domain))?;
        let req = self.apply_auth(self.client.post(url), auth);
        debug!("Request sent to rollback: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        let rollback_response: RollResponse = serde_json::from_str(&body_text)?;
        Ok(rollback_response)
    }

    /// Rolls forward a domain to a newer revision.
    ///
    /// # Arguments
    /// * `domain` - The domain to roll forward.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` indicating success or a `SurgeError`.
    pub async fn rollfore(&self, domain: &str, auth: &Auth) -> Result<RollResponse, SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/rollfore", domain))?;
        let req = self.apply_auth(self.client.post(url), auth);
        debug!("Request sent to rollfore: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        let rollfore_response: RollResponse = serde_json::from_str(&body_text)?;
        Ok(rollfore_response)
    }

    /// Switches a domain to a specific revision (or the latest if none specified).
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `revision` - Optional revision to switch to.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` indicating success or a `SurgeError`.
    pub async fn cutover(
        &self,
        domain: &str,
        revision: Option<&str>,
        auth: &Auth,
    ) -> Result<(), SurgeError> {
        let path = match revision {
            Some(rev) => format!("{}/rev/{}", domain, rev),
            None => format!("{}/rev", domain),
        };
        let url = self.config.endpoint.join(&path)?;
        let req = self.apply_auth(self.client.put(url), auth);
        debug!("Request sent to cutover: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        Ok(())
    }

    /// Discards a specific revision (or all revisions if none specified) for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `revision` - Optional revision to discard.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `DiscardResponse` indicating success or a `SurgeError`.
    pub async fn discard(
        &self,
        revision: &str,
        auth: &Auth,
    ) -> Result<DiscardResponse, SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/rev", revision))?;
        let req = self.apply_auth(self.client.delete(url), auth);
        debug!("Request sent to discard: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);

        let discard_response: DiscardResponse = serde_json::from_str(&body_text)?;
        Ok(discard_response)
    }

    /// Fetches SSL certificate information for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing a `DCertsResponse` or a `SurgeError`.
    pub async fn certs(&self, domain: &str, auth: &Auth) -> Result<CertsResponse, SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/certs", domain))?;
        let req = self.apply_auth(self.client.get(url), auth);
        debug!("Request sent to certs: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        let certs_response: CertsResponse = serde_json::from_str(&body_text)?;
        Ok(certs_response)
    }

    /// Fetches metadata for a domain or specific revision.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `revision` - Optional revision to fetch metadata for.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing a `MetadataResponse` or a `SurgeError`.
    pub async fn metadata(
        &self,
        domain: &str,
        revision: Option<&str>,
        auth: &Auth,
    ) -> Result<MetadataResponse, SurgeError> {
        let path = match revision {
            Some(rev) => format!("{}/{}/metadata.json", domain, rev),
            None => format!("{}/metadata.json", domain),
        };
        let url = self.config.endpoint.join(&path)?;
        let req = self.apply_auth(self.client.get(url), auth);
        debug!("Request sent to metadata: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        let metadata_response: MetadataResponse = serde_json::from_str(&body_text)?;
        Ok(metadata_response)
    }

    /// Fetches the manifest for a domain or specific revision.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `revision` - Optional revision to fetch the manifest for.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing a `ManifestResponse` or a `SurgeError`.
    pub async fn manifest(
        &self,
        domain: &str,
        revision: Option<&str>,
        auth: &Auth,
    ) -> Result<ManifestResponse, SurgeError> {
        let path = match revision {
            Some(rev) => format!("{}/{}/manifest.json", domain, rev),
            None => format!("{}/manifest.json", domain),
        };
        let url = self.config.endpoint.join(&path)?;
        let req = self.apply_auth(self.client.get(url), auth);
        debug!("Request sent to manifest: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        let manifest_response: ManifestResponse = serde_json::from_str(&body_text)?;
        Ok(manifest_response)
    }

    /// Fetches the file manifest for a domain (alias for `manifest` with no revision).
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing a `serde_json::Value` or a `SurgeError`.
    pub async fn files(&self, domain: &str, auth: &Auth) -> Result<ManifestResponse, SurgeError> {
        self.manifest(domain, None, auth).await
    }

    /// Updates configuration settings for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `settings` - JSON settings to apply.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` indicating success or a `SurgeError`.
    pub async fn config(
        &self,
        domain: &str,
        settings: Value,
        auth: &Auth,
    ) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/settings", domain))?;
        let req = self.apply_auth(self.client.put(url), auth).json(&settings);
        debug!("Request sent to config: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        Ok(())
    }

    /// Fetches DNS records for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing a `serde_json::Value` or a `SurgeError`.
    pub async fn dns(&self, domain: &str, auth: &Auth) -> Result<Value, SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/dns", domain))?;
        let req = self.apply_auth(self.client.get(url), auth);
        debug!("Request sent to dns: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        let dns_response: Value = serde_json::from_str(&body_text)?;
        Ok(dns_response)
    }

    /// Adds a DNS record for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `record` - JSON representation of the DNS record.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` indicating success or a `SurgeError`.
    pub async fn dns_add(
        &self,
        domain: &str,
        record: Value,
        auth: &Auth,
    ) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/dns", domain))?;
        let req = self.apply_auth(self.client.post(url), auth).json(&record);
        debug!("Request sent to dns_add: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        Ok(())
    }

    /// Removes a DNS record for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `id` - The ID of the DNS record to remove.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` indicating success or a `SurgeError`.
    pub async fn dns_remove(&self, domain: &str, id: &str, auth: &Auth) -> Result<(), SurgeError> {
        let url = self
            .config
            .endpoint
            .join(&format!("{}/dns/{}", domain, id))?;
        let req = self.apply_auth(self.client.delete(url), auth);
        debug!("Request sent to dns_remove: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        Ok(())
    }

    /// Fetches zone information for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing a `serde_json::Value` or a `SurgeError`.
    pub async fn zone(&self, domain: &str, auth: &Auth) -> Result<Value, SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/zone", domain))?;
        let req = self.apply_auth(self.client.get(url), auth);
        debug!("Request sent to zone: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        let zone_response: Value = serde_json::from_str(&body_text)?;
        Ok(zone_response)
    }

    /// Adds a zone record for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `record` - JSON representation of the zone record.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` indicating success or a `SurgeError`.
    pub async fn zone_add(
        &self,
        domain: &str,
        record: Value,
        auth: &Auth,
    ) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/zone", domain))?;
        let req = self.apply_auth(self.client.post(url), auth).json(&record);
        debug!("Request sent to zone_add: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        Ok(())
    }

    /// Removes a zone record for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `id` - The ID of the zone record to remove.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` indicating success or a `SurgeError`.
    pub async fn zone_remove(&self, domain: &str, id: &str, auth: &Auth) -> Result<(), SurgeError> {
        let url = self
            .config
            .endpoint
            .join(&format!("{}/zone/{}", domain, id))?;
        let req = self.apply_auth(self.client.delete(url), auth);
        debug!("Request sent to zone_remove: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        Ok(())
    }

    /// Clears the cache for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` indicating success or a `SurgeError`.
    pub async fn bust(&self, domain: &str, auth: &Auth) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/cache", domain))?;
        let req = self.apply_auth(self.client.delete(url), auth);
        debug!("Request sent to bust: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        Ok(())
    }

    /// Fetches account statistics.
    ///
    /// # Arguments
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing a `serde_json::Value` or a `SurgeError`.
    pub async fn stats(&self, auth: &Auth) -> Result<Value, SurgeError> {
        let url = self.config.endpoint.join("stats")?;
        let req = self.apply_auth(self.client.get(url), auth);
        debug!("Request sent to stats: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        let stats_response: Value = serde_json::from_str(&body_text)?;
        Ok(stats_response)
    }

    /// Fetches analytics data for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing a `DAnalyticsResponse` or a `SurgeError`.
    pub async fn analytics(
        &self,
        domain: &str,
        auth: &Auth,
    ) -> Result<DAnalyticsResponse, SurgeError> {
        let url = self
            .config
            .endpoint
            .join(&format!("{}/analytics", domain))?;
        let req = self.apply_auth(self.client.get(url), auth);
        debug!("Request sent to analytics: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        let analytics_response: DAnalyticsResponse = serde_json::from_str(&body_text)?;
        Ok(analytics_response)
    }

    /// Fetches usage data for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing a `DAnalyticsResponse` or a `SurgeError`.
    pub async fn usage(&self, domain: &str, auth: &Auth) -> Result<DAnalyticsResponse, SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/usage", domain))?;
        let req = self.apply_auth(self.client.get(url), auth);
        debug!("Request sent to usage: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        let usage_response = serde_json::from_str(&body_text)?;
        Ok(usage_response)
    }

    /// Fetches audit logs for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing a `DAuditResponse` or a `SurgeError`.
    pub async fn audit(&self, domain: &str, auth: &Auth) -> Result<DAuditResponse, SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/audit", domain))?;
        let req = self.apply_auth(self.client.get(url), auth);
        debug!("Request sent to audit: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        let audit_response = serde_json::from_str(&body_text)?;
        Ok(audit_response)
    }

    /// Invites collaborators to a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `emails` - JSON array of email addresses to invite.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `bool` indicating success or a `SurgeError`.
    pub async fn invite(
        &self,
        domain: &str,
        emails: Value,
        auth: &Auth,
    ) -> Result<bool, SurgeError> {
        let url = self
            .config
            .endpoint
            .join(&format!("{}/collaborators", domain))?;
        let req = self.apply_auth(self.client.post(url), auth).json(&emails);
        debug!("Request sent to invite: {:#?}", req);
        let res = req.send().await?;
        let status = res.status();
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        if status.is_success() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Revokes collaborator access for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `emails` - JSON array of email addresses to revoke.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `bool` indicating success or a `SurgeError`.
    pub async fn revoke(
        &self,
        domain: &str,
        emails: Value,
        auth: &Auth,
    ) -> Result<bool, SurgeError> {
        let url = self
            .config
            .endpoint
            .join(&format!("{}/collaborators", domain))?;
        let req = self.apply_auth(self.client.delete(url), auth).json(&emails);
        debug!("Request sent to revoke: {:#?}", req);
        let res = req.send().await?;
        let status = res.status();
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        if status.is_success() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Updates the account plan.
    ///
    /// # Arguments
    /// * `plan` - JSON representation of the plan.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` indicating success or a `SurgeError`.
    pub async fn plan(&self, plan: Value, auth: &Auth) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join("plan")?;
        let req = self.apply_auth(self.client.put(url), auth).json(&plan);
        debug!("Request sent to plan: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        Ok(())
    }

    /// Updates the payment card for the account.
    ///
    /// # Arguments
    /// * `card` - JSON representation of the card details.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` indicating success or a `SurgeError`.
    pub async fn card(&self, card: Value, auth: &Auth) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join("card")?;
        let req = self.apply_auth(self.client.put(url), auth).json(&card);
        debug!("Request sent to card: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        Ok(())
    }

    /// Fetches available plans, optionally for a specific domain.
    ///
    /// # Arguments
    /// * `domain` - Optional domain to filter plans.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` containing a `PlansResponse` or a `SurgeError`.
    pub async fn plans(
        &self,
        domain: Option<&str>,
        auth: &Auth,
    ) -> Result<PlansResponse, SurgeError> {
        let path = match domain {
            Some(d) => format!("{}/plans", d),
            None => "plans".to_string(),
        };
        let url = self.config.endpoint.join(&path)?;
        let req = self.apply_auth(self.client.get(url), auth);
        debug!("Request sent to plans: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        let plans_response: PlansResponse = serde_json::from_str(&body_text)?;
        Ok(plans_response)
    }

    /// Uploads an SSL certificate for a domain.
    ///
    /// # Arguments
    /// * `domain` - The target domain.
    /// * `pem_path` - Path to the PEM certificate file.
    /// * `auth` - Authentication credentials.
    ///
    /// # Returns
    /// A `Result` indicating success or a `SurgeError`.
    pub async fn ssl(&self, domain: &str, pem_path: &Path, auth: &Auth) -> Result<(), SurgeError> {
        let pem_data = fs::read(pem_path).map_err(|e| SurgeError::Io(e.to_string()))?;
        let url = self.config.endpoint.join(&format!("{}/certs", domain))?;
        let req = self.apply_auth(self.client.post(url), auth).body(pem_data);
        debug!("Request sent to ssl: {:#?}", req);
        let res = req.send().await?;
        let body_text = res.text().await?;
        debug!("response raw: {:?}", body_text);
        Ok(())
    }

    /// Applies authentication to an HTTP request.
    ///
    /// # Arguments
    /// * `req` - The `reqwest::RequestBuilder` to modify.
    /// * `auth` - Authentication credentials (token or username/password).
    ///
    /// # Returns
    /// The modified `RequestBuilder` with authentication headers.
    pub fn apply_auth(&self, req: reqwest::RequestBuilder, auth: &Auth) -> reqwest::RequestBuilder {
        match auth {
            Auth::Token(token) => req.basic_auth("token", Some(token)),
            Auth::UserPass { username, password } => req.basic_auth(username, Some(password)),
        }
    }
}

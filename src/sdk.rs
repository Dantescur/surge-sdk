/*
  src/client.rs
*/
use futures_util::{Stream, StreamExt};
use log::{debug, error, info, trace};
use ndjson_stream::{
    config::{EmptyLineHandling, NdjsonConfig},
    fallible::FallibleNdjsonError,
};
use serde_json::{Value, json};
use std::{fs, path::Path, time::Duration};

use reqwest::Client;

use crate::{
    DCertsResponse, ListResponse,
    config::Config,
    error::SurgeError,
    responses::{AccountResponse, LoginResponse},
    types::{Auth, Event},
};

pub struct SurgeSdk {
    pub config: Config,
    pub client: Client,
}

impl SurgeSdk {
    pub fn new(config: Config) -> Result<Self, SurgeError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .danger_accept_invalid_certs(config.insecure)
            .build()
            .map_err(SurgeError::Http)?;
        Ok(Self { config, client })
    }

    // Requires auth
    pub async fn account(&self, auth: &Auth) -> Result<AccountResponse, SurgeError> {
        let url = self.config.endpoint.join("account")?;
        debug!("Account url: {}", url);
        let req = self.apply_auth(self.client.get(url), auth);
        debug!("Request sended to account: {:#?}", req);
        let res = req.send().await?;
        let text = res.text().await?; // Get raw text response
        debug!("Raw response: {}", text); // Log raw response
        let parsed: AccountResponse = serde_json::from_str(&text)?; // Attempt deserialization
        debug!("Response received: {:#?}", parsed);
        Ok(parsed)
    }

    pub async fn list(
        &self,
        domain: Option<&str>,
        auth: &Auth,
    ) -> Result<ListResponse, SurgeError> {
        let path = match domain {
            Some(d) => format!("{}/list", d),
            None => "list".to_string(),
        };
        let url = self.config.endpoint.join(&path)?;
        let req = self.apply_auth(self.client.get(url), auth);
        let res = req.send().await?.json().await?;
        Ok(res)
    }

    pub async fn nuke(&self, auth: &Auth) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join("account")?;
        let req = self.apply_auth(self.client.delete(url), auth);
        req.send().await?;
        Ok(())
    }

    pub async fn teardown(&self, domain: &str, auth: &Auth) -> Result<bool, SurgeError> {
        let url = self.config.endpoint.join(domain)?;
        let req = self.apply_auth(self.client.delete(url), auth);
        let response = req.send().await?;

        Ok(response.status().is_success())
    }

    pub async fn login(&self, auth: &Auth) -> Result<LoginResponse, SurgeError> {
        let url = self.config.endpoint.join("token")?;
        let req = self.apply_auth(self.client.post(url), auth);
        let res = req.send().await?.json().await?;
        Ok(res)
    }

    // Streaming methods (implemented in stream.rs, called here)
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

    // Streaming methods (implemented in stream.rs, called here)
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

    pub async fn rollback(&self, domain: &str, auth: &Auth) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/rollback", domain))?;
        let req = self.apply_auth(self.client.post(url), auth);
        req.send().await?;
        Ok(())
    }

    pub async fn rollfore(&self, domain: &str, auth: &Auth) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/rollfore", domain))?;
        let req = self.apply_auth(self.client.post(url), auth);
        req.send().await?;
        Ok(())
    }

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
        req.send().await?;
        Ok(())
    }

    pub async fn discard(
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
        let req = self.apply_auth(self.client.delete(url), auth);
        req.send().await?;
        Ok(())
    }

    pub async fn certs(&self, domain: &str, auth: &Auth) -> Result<DCertsResponse, SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/certs", domain))?;
        let req = self.apply_auth(self.client.get(url), auth);
        let res = req.send().await?.json().await?;
        Ok(res)
    }

    pub async fn metadata(
        &self,
        domain: &str,
        revision: Option<&str>,
        auth: &Auth,
    ) -> Result<Value, SurgeError> {
        let path = match revision {
            Some(rev) => format!("{}/{}/metadata.json", domain, rev),
            None => format!("{}/metadata.json", domain),
        };
        let url = self.config.endpoint.join(&path)?;
        let req = self.apply_auth(self.client.get(url), auth);
        let res = req.send().await?.json().await?;
        Ok(res)
    }

    pub async fn manifest(
        &self,
        domain: &str,
        revision: Option<&str>,
        auth: &Auth,
    ) -> Result<Value, SurgeError> {
        let path = match revision {
            Some(rev) => format!("{}/{}/manifest.json", domain, rev),
            None => format!("{}/manifest.json", domain),
        };
        let url = self.config.endpoint.join(&path)?;
        let req = self.apply_auth(self.client.get(url), auth);
        let res = req.send().await?.json().await?;
        Ok(res)
    }

    pub async fn files(&self, domain: &str, auth: &Auth) -> Result<Value, SurgeError> {
        self.manifest(domain, None, auth).await
    }

    // Helper to apply authentication
    pub fn apply_auth(&self, req: reqwest::RequestBuilder, auth: &Auth) -> reqwest::RequestBuilder {
        match auth {
            Auth::Token(token) => req.basic_auth("token", Some(token)),
            Auth::UserPass { username, password } => req.basic_auth(username, Some(password)),
        }
    }

    pub async fn config(
        &self,
        domain: &str,
        settings: Value,
        auth: &Auth,
    ) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/settings", domain))?;
        let req = self.apply_auth(self.client.put(url), auth).json(&settings);
        req.send().await?;
        Ok(())
    }

    pub async fn dns(&self, domain: &str, auth: &Auth) -> Result<Value, SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/dns", domain))?;
        let req = self.apply_auth(self.client.get(url), auth);
        let res = req.send().await?.json().await?;
        Ok(res)
    }

    pub async fn dns_add(
        &self,
        domain: &str,
        record: Value,
        auth: &Auth,
    ) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/dns", domain))?;
        let req = self.apply_auth(self.client.post(url), auth).json(&record);
        req.send().await?;
        Ok(())
    }

    pub async fn dns_remove(&self, domain: &str, id: &str, auth: &Auth) -> Result<(), SurgeError> {
        let url = self
            .config
            .endpoint
            .join(&format!("{}/dns/{}", domain, id))?;
        let req = self.apply_auth(self.client.delete(url), auth);
        req.send().await?;
        Ok(())
    }

    pub async fn zone(&self, domain: &str, auth: &Auth) -> Result<Value, SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/zone", domain))?;
        let req = self.apply_auth(self.client.get(url), auth);
        let res = req.send().await?.json().await?;
        Ok(res)
    }

    pub async fn zone_add(
        &self,
        domain: &str,
        record: Value,
        auth: &Auth,
    ) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/zone", domain))?;
        let req = self.apply_auth(self.client.post(url), auth).json(&record);
        req.send().await?;
        Ok(())
    }

    pub async fn zone_remove(&self, domain: &str, id: &str, auth: &Auth) -> Result<(), SurgeError> {
        let url = self
            .config
            .endpoint
            .join(&format!("{}/zone/{}", domain, id))?;
        let req = self.apply_auth(self.client.delete(url), auth);
        req.send().await?;
        Ok(())
    }

    pub async fn bust(&self, domain: &str, auth: &Auth) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/cache", domain))?;
        let req = self.apply_auth(self.client.delete(url), auth);
        req.send().await?;
        Ok(())
    }

    pub async fn stats(&self, auth: &Auth) -> Result<Value, SurgeError> {
        let url = self.config.endpoint.join("stats")?;
        let req = self.apply_auth(self.client.get(url), auth);
        let res = req.send().await?.json().await?;
        Ok(res)
    }

    pub async fn analytics(&self, domain: &str, auth: &Auth) -> Result<Value, SurgeError> {
        let url = self
            .config
            .endpoint
            .join(&format!("{}/analytics", domain))?;
        let req = self.apply_auth(self.client.get(url), auth);
        let res = req.send().await?.json().await?;
        Ok(res)
    }

    pub async fn usage(&self, domain: &str, auth: &Auth) -> Result<Value, SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/usage", domain))?;
        let req = self.apply_auth(self.client.get(url), auth);
        let res = req.send().await?.json().await?;
        Ok(res)
    }

    pub async fn audit(&self, domain: &str, auth: &Auth) -> Result<Value, SurgeError> {
        let url = self.config.endpoint.join(&format!("{}/audit", domain))?;
        let req = self.apply_auth(self.client.get(url), auth);
        let res = req.send().await?.json().await?;
        Ok(res)
    }

    pub async fn invite(&self, domain: &str, emails: Value, auth: &Auth) -> Result<(), SurgeError> {
        let url = self
            .config
            .endpoint
            .join(&format!("{}/collaborators", domain))?;
        let req = self.apply_auth(self.client.post(url), auth).json(&emails);
        req.send().await?;
        Ok(())
    }

    pub async fn revoke(&self, domain: &str, emails: Value, auth: &Auth) -> Result<(), SurgeError> {
        let url = self
            .config
            .endpoint
            .join(&format!("{}/collaborators", domain))?;
        let req = self.apply_auth(self.client.delete(url), auth).json(&emails);
        req.send().await?;
        Ok(())
    }

    pub async fn plan(&self, plan: Value, auth: &Auth) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join("plan")?;
        let req = self.apply_auth(self.client.put(url), auth).json(&plan);
        req.send().await?;
        Ok(())
    }

    pub async fn card(&self, card: Value, auth: &Auth) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join("card")?;
        let req = self.apply_auth(self.client.put(url), auth).json(&card);
        req.send().await?;
        Ok(())
    }

    pub async fn plans(&self, domain: Option<&str>, auth: &Auth) -> Result<Value, SurgeError> {
        let path = match domain {
            Some(d) => format!("{}/plans", d),
            None => "plans".to_string(),
        };
        let url = self.config.endpoint.join(&path)?;
        let req = self.apply_auth(self.client.get(url), auth);
        let res = req.send().await?.json().await?;
        Ok(res)
    }

    pub async fn encrypt(
        &self,
        domain: &str,
        auth: &Auth,
        headers: Option<Vec<(String, String)>>,
        argv: Option<&[String]>,
    ) -> Result<impl Stream<Item = Result<Event, SurgeError>>, SurgeError> {
        info!("Encrypting domain: {}", domain);
        let url = self.config.endpoint.join(&format!("{}/encrypt", domain))?;
        debug!("URL: {}", url);

        let timestamp = chrono::Utc::now().to_rfc3339();
        let argv_json = argv.map_or_else(
            || {
                Ok(json!({
                    "_": [],
                    "e": self.config.endpoint.as_str(),
                    "endpoint": self.config.endpoint.as_str(),
                })
                .to_string())
            },
            |args| {
                serde_json::to_string(&json!({
                    "_": args,
                    "e": self.config.endpoint.as_str(),
                    "endpoint": self.config.endpoint.as_str(),
                }))
            },
        )?;

        let mut req = self
            .client
            .put(url)
            .header("Accept", "application/ndjson")
            .header("version", &self.config.version)
            .header("timestamp", timestamp)
            .header("argv", argv_json);

        if let Some(headers) = headers {
            debug!("Adding custom headers: {:?}", headers);
            for (key, value) in headers {
                req = req.header(&key, value);
            }
        }

        req = self.apply_auth(req, auth);

        debug!("Sending encrypt request");
        let res = req.send().await?;
        debug!("Response status: {}", res.status());

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await?;
            error!("Encrypt failed with status {}: {}", status, text);
            return Err(SurgeError::Api(crate::error::ApiError {
                errors: vec![format!("Encrypt failed with status: {}", status)],
                details: Value::Object(serde_json::Map::new()),
                status: Some(status.as_u16()),
            }));
        }

        info!("Encrypt request sent for domain: {}", domain);

        let stream = res.bytes_stream();
        let config =
            NdjsonConfig::default().with_empty_line_handling(EmptyLineHandling::IgnoreEmpty);

        let stream = stream.map(|result| {
            result.map_err(SurgeError::from).and_then(|bytes| {
                trace!("Received {} bytes", bytes.len());
                String::from_utf8(bytes.to_vec()).map_err(|e| {
                    error!("UTF-8 error: {}", e);
                    SurgeError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                })
            })
        });

        let ndjson_stream = ndjson_stream::from_fallible_stream_with_config(stream, config);

        Ok(ndjson_stream.map(|result: Result<Event, _>| match result {
            Ok(event) => {
                debug!("Parsed event: {:?}", event);
                if event.event_type == *"error" || event.data.to_string().contains("error") {
                    error!("Server error: {:?}", event);
                    Err(SurgeError::EventError(event))
                } else if event.event_type == *"info" {
                    info!("Success indicator received");
                    Ok(event)
                } else {
                    Ok(event)
                }
            }
            Err(FallibleNdjsonError::JsonError(e)) => {
                error!("JSON parsing error: {}", e);
                Err(SurgeError::Json(e))
            }
            Err(FallibleNdjsonError::InputError(e)) => {
                error!("Stream error: {:?}", e);
                Err(SurgeError::Io(std::io::Error::other(e.to_string())))
            }
        }))
    }

    pub async fn ssl(&self, domain: &str, pem_path: &Path, auth: &Auth) -> Result<(), SurgeError> {
        let pem_data = fs::read(pem_path).map_err(|e| SurgeError::Io(std::io::Error::other(e)))?;
        let url = self.config.endpoint.join(&format!("{}/certs", domain))?;
        let req = self.apply_auth(self.client.post(url), auth).body(pem_data);
        req.send().await?;
        Ok(())
    }
}

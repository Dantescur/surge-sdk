/*
  src/client.rs
*/
use std::{path::Path, time::Duration};

use futures::Stream;
use reqwest::Client;

use crate::{
    ListResponse,
    config::Config,
    error::SurgeError,
    responses::{AccountResponse, LoginResponse},
    types::{Auth, Event},
};

pub struct SurgeClient {
    pub config: Config,
    pub client: Client,
}

impl SurgeClient {
    pub fn new(config: Config) -> Result<Self, SurgeError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .danger_accept_invalid_certs(config.insecure)
            .build()
            .map_err(SurgeError::Http)?;
        Ok(Self { config, client })
    }

    // Requires auth
    pub async fn account(&self, auth: Auth) -> Result<AccountResponse, SurgeError> {
        let url = self.config.endpoint.join("account")?;
        let req = self.apply_auth(self.client.get(url), auth);
        let res = req.send().await?.json().await?;
        Ok(res)
    }

    pub async fn list(&self, auth: Auth) -> Result<ListResponse, SurgeError> {
        let url = self.config.endpoint.join("list")?;
        let req = self.apply_auth(self.client.get(url), auth);
        let res = req.send().await?.json().await?;
        Ok(res)
    }

    pub async fn nuke(&self, auth: Auth) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join("account")?;
        let req = self.apply_auth(self.client.delete(url), auth);
        req.send().await?;
        Ok(())
    }

    pub async fn teardown(&self, domain: &str, auth: Auth) -> Result<(), SurgeError> {
        let url = self.config.endpoint.join(&format!("domain/{}", domain))?;
        let req = self.apply_auth(self.client.delete(url), auth);
        req.send().await?;
        Ok(())
    }

    pub async fn login(&self, auth: Auth) -> Result<LoginResponse, SurgeError> {
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
        auth: Auth,
        headers: Option<Vec<(String, String)>>,
        argv: Option<&[String]>,
    ) -> Result<impl Stream<Item = Result<Event, SurgeError>>, SurgeError> {
        crate::stream::publish(self, project_path, domain, auth, headers, argv).await
    }

    // Helper to apply authentication
    pub fn apply_auth(&self, req: reqwest::RequestBuilder, auth: Auth) -> reqwest::RequestBuilder {
        match auth {
            Auth::Token(token) => req.basic_auth("token", Some(token)),
            Auth::UserPass { username, password } => req.basic_auth(username, Some(password)),
        }
    }
}

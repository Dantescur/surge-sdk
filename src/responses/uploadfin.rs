/*
  src/responses/uploadfin.rs
*/
/*
  responses/uploadfin.rs
*/
use serde::{Deserialize, Serialize};

use super::{Cert, Instance, Metadata, Url};

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadFinResponse {
    #[serde(rename = "type")]
    pub upload_fin_response_type: String,
    pub config: UploadFinResponseConfig,
    pub certs: Vec<Cert>,
    pub metadata: Metadata,
    pub urls: Vec<Url>,
    pub instances: Vec<Instance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadFinResponseConfig {
    pub force: Option<serde_json::Value>,
    pub redirect: Option<serde_json::Value>,
    pub cors: Option<serde_json::Value>,
    pub hsts: Option<serde_json::Value>,
    pub ttl: Option<serde_json::Value>,
}

/*
  responses/manifest.rs
*/
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ManifestResponse = HashMap<String, ManifestResponseValue>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestResponseValue {
    pub size: i64,
    #[serde(rename = "md5sum")]
    pub md5_sum: String,
    #[serde(rename = "sha256sum")]
    pub sha256_sum: String,
}

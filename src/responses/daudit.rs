/*
  src/responses/daudit.rs
*/
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type DAuditResponse = HashMap<String, DAuditResponseValue>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DAuditResponseValue {
    #[serde(default)]
    pub rev: i64,
    #[serde(default)]
    pub private_file_list: Vec<Option<serde_json::Value>>,
    #[serde(default)]
    pub public_file_count: i64,
    #[serde(default)]
    pub public_total_size: i64,
    #[serde(default)]
    pub private_file_count: i64,
    #[serde(default)]
    pub private_total_size: i64,
    #[serde(default)]
    pub manifest: HashMap<String, Manifest>,
    #[serde(default)]
    pub cert: Option<Cert>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cert {
    #[serde(default)]
    pub subject: Option<Subject>,
    #[serde(default)]
    pub issuer: Option<Issuer>,
    #[serde(default)]
    pub subjectaltname: Option<String>,
    #[serde(rename = "infoAccess", default)]
    pub info_access: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub modulus: Option<String>,
    #[serde(default)]
    pub bits: Option<i64>,
    #[serde(default)]
    pub exponent: Option<String>,
    #[serde(default)]
    pub pubkey: Option<Pubkey>,
    #[serde(rename = "valid_from", default)]
    pub valid_from: Option<String>,
    #[serde(rename = "valid_to", default)]
    pub valid_to: Option<String>,
    #[serde(default)]
    pub fingerprint: Option<String>,
    #[serde(rename = "fingerprint256", default)]
    pub fingerprint256: Option<String>,
    #[serde(rename = "ext_key_usage", default)]
    pub ext_key_usage: Vec<String>,
    #[serde(rename = "serialNumber", default)]
    pub serial_number: Option<String>,
    #[serde(default)]
    pub raw: Option<Pubkey>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pubkey {
    #[serde(rename = "type", default)]
    pub key_type: Option<String>,
    #[serde(default)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Issuer {
    #[serde(default)]
    pub c: Option<String>,
    #[serde(default)]
    pub st: Option<String>,
    #[serde(default)]
    pub l: Option<String>,
    #[serde(default)]
    pub o: Option<String>,
    #[serde(default)]
    pub cn: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Subject {
    #[serde(default)]
    pub cn: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(default)]
    pub size: i64,
    #[serde(rename = "md5sum", default)]
    pub md5_sum: Option<String>,
    #[serde(rename = "sha256sum", default)]
    pub sha256_sum: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}


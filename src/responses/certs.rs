/*
  src/responses/certs.rs
*/
use chrono::DateTime;
use chrono::Utc;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertsResponse {
    pub certs: Vec<Cert>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cert {
    pub subject: String,
    pub issuer: String,
    pub not_before: String,
    pub not_after: DateTime<Utc>,
    pub exp_in_days: i64,
    pub subject_alt_names: Vec<String>,
    pub cert_name: String,
    pub auto_renew: bool,
}

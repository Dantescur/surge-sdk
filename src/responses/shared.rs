// src/responses/shared.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "type")]
    pub metadata_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cert {
    pub subject: String,
    pub issuer: String,
    pub not_before: String,
    pub not_after: String,
    pub exp_in_days: i64,
    pub subject_alt_names: Vec<String>,
    pub cert_name: String,
    pub auto_renew: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub amount: String,
    pub friendly: String,
    pub dummy: bool,
    pub current: bool,
    pub metadata: Metadata,
    pub ext: String,
    pub perks: Vec<String>,
    pub comped: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Amount {
    Integer(i64),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    pub name: String,
    pub domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputClass {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonMetadata {
    pub rev: i64,
    pub cmd: String,
    pub email: String,
    pub platform: String,
    pub cli_version: String,
    pub output: OutputClass,
    pub config: OutputClass,
    pub message: Option<serde_json::Value>,
    pub build_time: Option<serde_json::Value>,
    pub ip: String,
    pub private_file_list: Vec<Option<serde_json::Value>>,
    pub public_file_count: i64,
    pub public_total_size: i64,
    pub private_file_count: i64,
    pub private_total_size: i64,
    pub upload_start_time: i64,
    pub upload_end_time: i64,
    pub upload_duration: f64,
    pub current: Option<bool>,
    #[serde(default)]
    pub preview: Option<String>,
    #[serde(default)]
    pub time_ago_in_words: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    #[serde(rename = "type")]
    pub instance_type: Type,
    pub provider: Option<Provider>,
    pub domain: String,
    pub location: String,
    pub status: Status,
    pub status_color: Color,
    pub confirmation: Confirmation,
    pub confirmation_color: Color,
    pub ip: String,
    pub info: Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Confirmation {
    #[serde(rename = "✔")]
    Empty,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    Green,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Info {
    Available,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "CNAME")]
    Cname,
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "NS")]
    Ns,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "D.Ocean")]
    DOcean,
    Linode,
    Vultr,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "◍")]
    Empty,
}

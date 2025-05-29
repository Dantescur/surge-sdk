use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "type")]
    pub metadata_type: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cert {
    pub subject: String,
    pub issuer: String,
    pub not_before: chrono::DateTime<chrono::Utc>,
    pub not_after: chrono::DateTime<chrono::Utc>,
    pub exp_in_days: u32,
    pub subject_alt_names: Vec<String>,
    pub cert_name: String,
    pub auto_renew: bool,
    #[serde(default)]
    pub fingerprint: Option<String>,
    #[serde(default)]
    pub key_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub id: String,
    pub name: String,
    #[serde(flatten)]
    pub amount: Amount,
    pub friendly: String,
    pub dummy: bool,
    pub current: bool,
    pub metadata: Metadata,
    #[serde(default)]
    pub ext: Option<String>,
    #[serde(default)]
    pub perks: Vec<String>,
    #[serde(default)]
    pub comped: bool,
    #[serde(default)]
    pub features: HashMap<String, bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Amount {
    Integer(u64),
    Float(f64),
    String(String),
    Object(HashMap<String, serde_json::Value>),
}

impl Amount {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Amount::Integer(i) => Some(*i as f64),
            Amount::Float(f) => Some(*f),
            Amount::String(s) => s.parse().ok(),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    pub name: String,
    pub domain: String,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Output {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonMetadata {
    pub rev: u64,
    pub cmd: String,
    pub email: String,
    pub platform: String,
    pub cli_version: semver::Version,
    pub output: Output,
    pub config: Output,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub build_time: Option<f64>,
    pub ip: String,
    #[serde(default)]
    pub private_file_list: Vec<serde_json::Value>,
    pub public_file_count: u64,
    pub public_total_size: u64,
    pub private_file_count: u64,
    pub private_total_size: u64,
    pub upload_start_time: i64,
    pub upload_end_time: i64,
    pub upload_duration: f64,
    #[serde(default)]
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
    pub instance_type: InstanceType,
    #[serde(default)]
    pub provider: Option<Provider>,
    pub domain: String,
    #[serde(default)]
    pub location: Option<String>,
    pub status: Status,
    #[serde(default)]
    pub status_color: Option<Color>,
    #[serde(default)]
    pub confirmation: Option<Confirmation>,
    #[serde(default)]
    pub confirmation_color: Option<Color>,
    pub ip: String,
    pub info: Info,
    #[serde(default)]
    pub port: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confirmation {
    Checkmark,
    Text(String),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    Green,
    Red,
    Yellow,
    Blue,
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Info {
    Available,
    Unavailable,
    Maintenance,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum InstanceType {
    #[serde(rename = "CNAME")]
    Cname,
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "HTTPS")]
    Https,
    #[serde(rename = "NS")]
    Ns,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "TXT")]
    Txt,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(skip_serializing, skip_deserializing)]
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Provider {
    DigitalOcean,
    Linode,
    Vultr,
    Aws,
    Gcp,
    Azure,
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Active,
    Inactive,
    Pending,
    Error,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeAccount {
    pub id: String,
    pub object: String,
    #[serde(default)]
    pub account_balance: i64,
    #[serde(default)]
    pub address: Option<Address>,
    #[serde(default)]
    pub balance: i64,
    pub created: i64,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub default_currency: Option<String>,
    #[serde(default)]
    pub default_source: Option<String>,
    #[serde(default)]
    pub delinquent: bool,
    #[serde(default)]
    pub description: Option<String>,
    pub email: String,
    #[serde(default)]
    pub invoice_prefix: Option<String>,
    pub invoice_settings: Option<InvoiceSettings>,
    #[serde(default)]
    pub livemode: bool,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub next_invoice_sequence: i64,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub tax_exempt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceSettings {
    #[serde(default)]
    pub custom_fields: Vec<CustomField>,
    #[serde(default)]
    pub default_payment_method: Option<String>,
    #[serde(default)]
    pub footer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedList<T> {
    pub object: String,
    pub data: Vec<T>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub total_count: u64,
    #[serde(default)]
    pub url: Option<String>,
}

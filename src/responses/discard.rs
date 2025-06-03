use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscardResponse {
    pub rev: String,
    pub domain: String,
    pub uncached: Uncached,
    pub revision: Revision,
    pub instances: Vec<Instance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uncached {
    pub revs: Vec<String>,
    pub domains: Vec<String>,
    pub change: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Revision {
    pub rev: i64,
    pub cmd: String,
    pub email: String,
    pub platform: String,
    pub cli_version: String,
    pub output: Option<Value>,
    pub config: Config,
    pub message: Value,
    pub build_time: Value,
    pub ip: String,
    pub private_file_list: Vec<Value>,
    pub public_file_count: i64,
    pub public_total_size: i64,
    pub private_file_count: i64,
    pub private_total_size: i64,
    pub upload_start_time: i64,
    pub upload_end_time: i64,
    pub upload_duration: f64,
    pub preview: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub pdf: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    #[serde(rename = "type")]
    pub type_field: String,
    pub provider: Option<String>,
    pub domain: String,
    pub location: String,
    pub status: String,
    pub status_color: String,
    pub confirmation: String,
    pub confirmation_color: String,
    pub ip: String,
    pub info: String,
}

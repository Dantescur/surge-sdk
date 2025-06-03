/*
  src/responses/list.rs
*/
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub enum ListResult {
    Global(Vec<ListResponse>),
    Domain(ListDomainResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse {
    pub domain: String,
    pub plan_name: String,
    pub rev: u64,
    pub cmd: String,
    pub email: String,
    pub platform: String,
    #[serde(rename = "cliVersion")]
    pub cli_version: String,
    pub output: Value,
    pub config: Value,
    pub message: Option<String>,
    #[serde(rename = "buildTime")]
    pub build_time: Option<String>,
    pub ip: String,
    #[serde(rename = "privateFileList")]
    pub private_file_list: Vec<String>,
    #[serde(rename = "publicFileCount")]
    pub public_file_count: u64,
    #[serde(rename = "publicTotalSize")]
    pub public_total_size: u64,
    #[serde(rename = "privateFileCount")]
    pub private_file_count: u64,
    #[serde(rename = "privateTotalSize")]
    pub private_total_size: u64,
    #[serde(rename = "uploadStartTime")]
    pub upload_start_time: u64,
    #[serde(rename = "uploadEndTime")]
    pub upload_end_time: u64,
    #[serde(rename = "plansuploadDuratiod")]
    pub plansupload_duratiod: f64,
    pub preview: Option<String>,
    #[serde(rename = "timeAgoInWords")]
    pub time_ago_in_words: String,
}

pub type ListDomainResponse = Vec<DomainList>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainList {
    pub rev: i64,
    pub platform: String,
    pub email: String,
    pub cmd: String,
    pub public_file_count: i64,
    pub public_total_size: i64,
    pub build_time: Value,
    pub msg: Value,
    pub current: bool,
    pub preview: String,
    pub friendly_size: String,
    pub time_ago_in_words: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub amount: String,
    pub friendly: String,
    pub dummy: bool,
    pub current: bool,
    pub metadata: Metadata2,
    pub ext: String,
    pub perks: Vec<String>,
    pub comped: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub extra: Option<String>, // Added to handle "extra" field in mock
}

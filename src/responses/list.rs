/*
  src/responses/list.rs
*/
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse {
    pub domain: String,
    pub plan_name: String,
    pub rev: u64,
    pub cmd: String,
    pub email: String,
    pub platform: String,
    pub cliVersion: String,
    pub output: serde_json::Value,
    pub config: serde_json::Value,
    pub message: Option<String>,
    pub buildTime: Option<String>,
    pub ip: String,
    pub privateFileList: Vec<String>,
    pub publicFileCount: u64,
    pub publicTotalSize: u64,
    pub privateFileCount: u64,
    pub privateTotalSize: u64,
    pub uploadStartTime: u64,
    pub uploadEndTime: u64,
    pub uploadDuration: f64,
    pub preview: String,
    pub timeAgoInWords: String,
}

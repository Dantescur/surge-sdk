/*
  src/responses/list.rs
*/
use super::{Output, Plan};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ListResponse = Vec<Deployment>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    pub domain: String,
    pub plan_name: Plan,
    pub rev: u64,
    pub cmd: Command,
    pub email: String,
    pub platform: String,
    pub cli_version: String,
    pub output: Option<Output>,
    pub config: Option<Config>,
    pub message: Option<String>,
    pub build_time: Option<f64>,
    pub ip: String,
    pub private_file_list: Vec<Option<serde_json::Value>>,
    pub public_file_count: u64,
    pub public_total_size: u64,
    pub private_file_count: u64,
    pub private_total_size: u64,
    pub upload_start_time: u64,
    pub upload_end_time: u64,
    pub upload_duration: f64,
    pub preview: Option<String>,
    pub time_ago_in_words: String,
    #[serde(flatten)]
    extra_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    Surge,
    Teardown,
    List,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub settings: HashMap<String, serde_json::Value>,
}

/*
  src/responses/metadata.rs
*/
use serde::{Deserialize, Serialize};
use serde_json::Value; // For the flexible "output" field

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataResponse {
    pub rev: i64,
    pub cmd: String,
    pub email: String,
    pub platform: String,
    pub cli_version: String,
    pub output: Value, // Using Value for flexible JSON object
    pub config: Config,
    pub message: Option<String>,    // Nullable field
    pub build_time: Option<String>, // Nullable field
    pub ip: String,
    pub private_file_list: Vec<Value>,
    pub public_file_count: i32,
    pub public_total_size: i32,
    pub private_file_count: i32,
    pub private_total_size: i32,
    pub upload_start_time: i64,
    pub upload_end_time: i64,
    pub upload_duration: f64,
    pub preview: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub pdf: bool,
}

/*
  src/responses/danalytics.rs
*/
/*
  responses/danalytics.rs
*/
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DAnalyticsResponse {
    pub normalized_at: String,
    pub version: String,
    pub domain: String,
    pub range: Vec<String>,
    pub traffic: Traffic,
    pub encryption: Encryption,
    pub bandwidth: Bandwidth,
    pub cache: Cache,
    pub source: HashMap<String, Vec<Option<serde_json::Value>>>,
    pub device: HashMap<String, Vec<Option<serde_json::Value>>>,
    pub os: HashMap<String, Vec<Option<serde_json::Value>>>,
    pub browser: HashMap<String, Vec<Option<serde_json::Value>>>,
    pub success: HashMap<String, Vec<Option<serde_json::Value>>>,
    pub fail: HashMap<String, Vec<Option<serde_json::Value>>>,
    pub redirect: HashMap<String, Vec<Option<serde_json::Value>>>,
    pub load: HashMap<String, Vec<Option<serde_json::Value>>>,
    pub datacenters: HashMap<String, Datacenter>,
    pub normalized_at_in_words: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bandwidth {
    pub all: All,
    pub body: All,
    pub headers: All,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct All {
    pub t: i64,
    pub s: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub hit: All,
    pub miss: All,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datacenter {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encryption {
    pub c_e: All,
    pub c_u: All,
    pub c_re: All,
    pub c_ru: All,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Traffic {
    pub connections: All,
    pub visits: All,
    pub uniques: All,
}

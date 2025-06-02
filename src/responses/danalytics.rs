/*
  src/responses/danalytics.rs
*/
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DAnalyticsResponse {
    #[serde(default)]
    pub normalized_at: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub range: Vec<String>,

    #[serde(default)]
    pub traffic: Option<Traffic>,
    #[serde(default)]
    pub encryption: Option<Encryption>,
    #[serde(default)]
    pub bandwidth: Option<Bandwidth>,
    #[serde(default)]
    pub cache: Option<Cache>,

    #[serde(default)]
    pub source: HashMap<String, Vec<Option<serde_json::Value>>>,
    #[serde(default)]
    pub device: HashMap<String, Vec<Option<serde_json::Value>>>,
    #[serde(default)]
    pub os: HashMap<String, Vec<Option<serde_json::Value>>>,
    #[serde(default)]
    pub browser: HashMap<String, Vec<Option<serde_json::Value>>>,
    #[serde(default)]
    pub success: HashMap<String, Vec<Option<serde_json::Value>>>,
    #[serde(default)]
    pub fail: HashMap<String, Vec<Option<serde_json::Value>>>,
    #[serde(default)]
    pub redirect: HashMap<String, Vec<Option<serde_json::Value>>>,
    #[serde(default)]
    pub load: HashMap<String, Vec<Option<serde_json::Value>>>,
    #[serde(default)]
    pub datacenters: HashMap<String, Datacenter>,

    #[serde(default)]
    pub normalized_at_in_words: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeries {
    #[serde(default)]
    pub t: i64,
    #[serde(default)]
    pub s: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bandwidth {
    #[serde(default)]
    pub all: TimeSeries,
    #[serde(default)]
    pub body: TimeSeries,
    #[serde(default)]
    pub headers: TimeSeries,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    #[serde(default)]
    pub hit: TimeSeries,
    #[serde(default)]
    pub miss: TimeSeries,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Datacenter {
    #[serde(default)]
    pub t: i64,
    #[serde(default)]
    pub s: Vec<i64>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encryption {
    #[serde(rename = "c_e", default)]
    pub encrypted: TimeSeries,
    #[serde(rename = "c_u", default)]
    pub unencrypted: TimeSeries,
    #[serde(rename = "c_re", default)]
    pub requested_encrypted: TimeSeries,
    #[serde(rename = "c_ru", default)]
    pub requested_unencrypted: TimeSeries,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Traffic {
    #[serde(default)]
    pub connections: TimeSeries,
    #[serde(default)]
    pub visits: TimeSeries,
    #[serde(default)]
    pub uniques: TimeSeries,
}


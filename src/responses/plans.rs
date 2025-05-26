/*
  src/responses/plans.rs
*/
/*
  responses/plans.rs
*/
use serde::{Deserialize, Serialize};

use super::{Amount, Metadata};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlansResponse {
    pub stripe_pk: String,
    pub current: Option<serde_json::Value>,
    pub list: Vec<List>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub id: String,
    pub name: String,
    pub amount: Amount,
    pub friendly: String,
    pub dummy: Option<bool>,
    pub current: bool,
    pub metadata: Metadata,
    pub ext: String,
    pub perks: Vec<String>,
    pub comped: bool,
    pub object: Option<String>,
    pub active: Option<bool>,
    pub aggregate_usage: Option<serde_json::Value>,
    pub amount_decimal: Option<String>,
    pub billing_scheme: Option<String>,
    pub created: Option<i64>,
    pub currency: Option<String>,
    pub interval: Option<String>,
    pub interval_count: Option<i64>,
    pub livemode: Option<bool>,
    pub meter: Option<serde_json::Value>,
    pub nickname: Option<serde_json::Value>,
    pub product: Option<String>,
    pub statement_descriptor: Option<serde_json::Value>,
    pub tiers: Option<serde_json::Value>,
    pub tiers_mode: Option<serde_json::Value>,
    pub transform_usage: Option<serde_json::Value>,
    pub trial_period_days: Option<i64>,
    pub usage_type: Option<String>,
}

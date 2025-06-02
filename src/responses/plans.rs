/*
  src/responses/plans.rs
*/
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlansResponse {
    pub stripe_pk: String,
    pub current: Option<Plan>,
    pub list: Vec<Plan>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub friendly: String,
    pub ext: String,
    pub perks: Vec<String>,
    pub comped: bool,
    pub current: bool,
    pub metadata: HashMap<String, String>,

    // Optional fields (only present in paid plans)
    pub dummy: Option<bool>,
    pub object: Option<String>,
    pub active: Option<bool>,
    pub aggregate_usage: Option<String>,
    pub amount: Option<u64>, // sometimes string, sometimes number, we choose u64 and fallback parse if needed
    pub amount_decimal: Option<String>,
    pub billing_scheme: Option<String>,
    pub created: Option<u64>,
    pub currency: Option<String>,
    pub interval: Option<String>,
    pub interval_count: Option<u8>,
    pub livemode: Option<bool>,
    pub meter: Option<String>,
    pub nickname: Option<String>,
    pub product: Option<String>,
    pub statement_descriptor: Option<String>,
    pub tiers: Option<serde_json::Value>,
    pub tiers_mode: Option<String>,
    pub transform_usage: Option<serde_json::Value>,
    pub trial_period_days: Option<u8>,
    pub usage_type: Option<String>,
}

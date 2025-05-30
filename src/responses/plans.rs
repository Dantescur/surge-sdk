use std::collections::HashMap;

/*
  responses/plans.rs
*/
use serde::{Deserialize, Serialize};

use super::{Amount, Metadata};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlansResponse {
    #[serde(default)]
    pub stripe_pk: Option<String>,
    #[serde(default)]
    pub current: Option<serde_json::Value>,
    #[serde(default)]
    pub list: Vec<Plan>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub amount: Amount,
    pub friendly: String,
    #[serde(default)]
    pub dummy: Option<bool>,
    #[serde(default)]
    pub current: bool,
    pub metadata: Metadata,
    #[serde(default)]
    pub ext: Option<String>,
    #[serde(default)]
    pub perks: Vec<String>,
    #[serde(default)]
    pub comped: bool,

    // Optional Stripe-specific fields
    #[serde(default)]
    pub object: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub aggregate_usage: Option<serde_json::Value>,
    #[serde(default)]
    pub amount_decimal: Option<String>,
    #[serde(default)]
    pub billing_scheme: Option<String>,
    #[serde(default)]
    pub created: Option<i64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub interval: Option<String>,
    #[serde(default)]
    pub interval_count: Option<i64>,
    #[serde(default)]
    pub livemode: Option<bool>,
    #[serde(default)]
    pub meter: Option<serde_json::Value>,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub product: Option<String>,
    #[serde(default)]
    pub statement_descriptor: Option<String>,
    #[serde(default)]
    pub tiers: Option<serde_json::Value>,
    #[serde(default)]
    pub tiers_mode: Option<String>,
    #[serde(default)]
    pub transform_usage: Option<serde_json::Value>,
    #[serde(default)]
    pub trial_period_days: Option<i64>,
    #[serde(default)]
    pub usage_type: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

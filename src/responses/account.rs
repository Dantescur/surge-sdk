/*
  src/responses/account.rs
*/
use serde::{Deserialize, Serialize};

use super::StripeAccount;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountResponse {
    pub email: String,
    pub id: String,
    pub uuid: String,
    pub role: i64,
    pub updated_at: String,
    pub created_at: String,
    pub payment_id: Option<String>,
    pub email_verified_at: Option<serde_json::Value>,
    pub stripe: Option<StripeAccount>,
    pub plan: Plan,
    pub card: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub amount: String,
    pub friendly: String,
    pub dummy: bool,
    pub current: bool,
    pub metadata: PlanMetadata,
    pub ext: String,
    pub perks: Vec<String>,
    pub comped: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanMetadata {
    #[serde(rename = "type")]
    pub metadata_type: String,
}

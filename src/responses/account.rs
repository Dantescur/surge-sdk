/*
  src/responses/account.rs
*/
/*
  responses/account.rs
*/
use serde::{Deserialize, Serialize};

use super::Plan;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    pub email: String,
    pub id: String,
    pub uuid: String,
    pub role: i64,
    pub updated_at: String,
    pub created_at: String,
    pub email_verified_at: Option<serde_json::Value>,
    pub payment_id: Option<serde_json::Value>,
    pub plan: Plan,
    pub card: Option<serde_json::Value>,
}

/*
  src/responses/login.rs
*/
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub email: String,
    pub token: String,
}

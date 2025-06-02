/*
  src/responses/stripe.rs
*/
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StripeToken {
    pub id: String,
    pub object: String,
    pub card: StripeCard,
    pub client_ip: String,
    pub created: u64,
    pub livemode: bool,
    #[serde(rename = "type")]
    pub token_type: String,
    pub used: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StripeCard {
    pub id: String,
    pub object: String,
    pub address_city: Option<String>,
    pub address_country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line1_check: Option<String>,
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub address_zip_check: Option<String>,
    pub brand: String,
    pub country: String,
    pub cvc_check: Option<String>,
    pub dynamic_last4: Option<String>,
    pub exp_month: u8,
    pub exp_year: u16,
    pub funding: String,
    pub last4: String,
    pub name: Option<String>,
    pub networks: StripeNetworks,
    pub regulated_status: Option<String>,
    pub tokenization_method: Option<String>,
    pub wallet: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StripeNetworks {
    pub preferred: Option<String>,
}

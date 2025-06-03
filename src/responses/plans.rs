use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlansResponse {
    pub card: Card,
    #[serde(rename = "stripe_pk")]
    pub stripe_pk: String,
    pub current: Value,
    pub list: Vec<List>,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: String,
    pub object: String,
    #[serde(rename = "address_city")]
    pub address_city: Value,
    #[serde(rename = "address_country")]
    pub address_country: Value,
    #[serde(rename = "address_line1")]
    pub address_line1: Value,
    #[serde(rename = "address_line1_check")]
    pub address_line1_check: Value,
    #[serde(rename = "address_line2")]
    pub address_line2: Value,
    #[serde(rename = "address_state")]
    pub address_state: Value,
    #[serde(rename = "address_zip")]
    pub address_zip: Value,
    #[serde(rename = "address_zip_check")]
    pub address_zip_check: Value,
    #[serde(rename = "allow_redisplay")]
    pub allow_redisplay: String,
    pub brand: String,
    pub country: String,
    pub customer: String,
    #[serde(rename = "cvc_check")]
    pub cvc_check: String,
    #[serde(rename = "dynamic_last4")]
    pub dynamic_last4: Value,
    #[serde(rename = "exp_month")]
    pub exp_month: i64,
    #[serde(rename = "exp_year")]
    pub exp_year: i64,
    pub fingerprint: String,
    pub funding: String,
    pub last4: String,
    pub metadata: Value,
    pub name: Value,
    #[serde(rename = "regulated_status")]
    pub regulated_status: String,
    #[serde(rename = "tokenization_method")]
    pub tokenization_method: Value,
    pub wallet: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
    pub id: String,
    pub name: String,
    pub amount: Value,
    pub friendly: String,
    pub dummy: Option<bool>,
    pub current: bool,
    pub metadata: Metadata2,
    pub ext: String,
    pub perks: Vec<String>,
    pub object: Option<String>,
    pub active: Option<bool>,
    #[serde(rename = "aggregate_usage")]
    pub aggregate_usage: Option<String>,
    #[serde(rename = "amount_decimal")]
    pub amount_decimal: Option<String>,
    #[serde(rename = "billing_scheme")]
    pub billing_scheme: Option<String>,
    pub created: Option<i64>,
    pub currency: Option<String>,
    pub interval: Option<String>,
    #[serde(rename = "interval_count")]
    pub interval_count: Option<i64>,
    pub livemode: Option<bool>,
    pub meter: Option<String>,
    pub nickname: Option<String>,
    pub product: Option<String>,
    #[serde(rename = "statement_descriptor")]
    pub statement_descriptor: Option<String>,
    pub tiers: Option<String>,
    #[serde(rename = "tiers_mode")]
    pub tiers_mode: Option<String>,
    #[serde(rename = "transform_usage")]
    pub transform_usage: Option<String>,
    #[serde(rename = "trial_period_days")]
    pub trial_period_days: Option<String>,
    #[serde(rename = "usage_type")]
    pub usage_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata2 {
    pub projects: Option<String>,
    pub preview: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
}

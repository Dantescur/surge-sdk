/*
  src/responses/account.rs
*/
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountResponse {
    pub email: String,
    pub id: String,
    pub uuid: String,
    pub role: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "payment_id")]
    pub payment_id: String,
    #[serde(rename = "email_verified_at")]
    pub email_verified_at: Value,
    pub stripe: Stripe,
    pub plan: Plan,
    pub card: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stripe {
    pub id: String,
    pub object: String,
    #[serde(rename = "account_balance")]
    pub account_balance: i64,
    pub address: Value,
    pub balance: i64,
    pub created: i64,
    pub currency: Value,
    #[serde(rename = "default_currency")]
    pub default_currency: Value,
    #[serde(rename = "default_source")]
    pub default_source: Value,
    pub delinquent: bool,
    pub description: Value,
    pub discount: Value,
    pub email: String,
    #[serde(rename = "invoice_prefix")]
    pub invoice_prefix: String,
    #[serde(rename = "invoice_settings")]
    pub invoice_settings: InvoiceSettings,
    pub livemode: bool,
    pub metadata: Metadata,
    pub name: Value,
    #[serde(rename = "next_invoice_sequence")]
    pub next_invoice_sequence: i64,
    pub phone: Value,
    #[serde(rename = "preferred_locales")]
    pub preferred_locales: Vec<Value>,
    pub shipping: Value,
    pub sources: Sources,
    pub subscriptions: Subscriptions,
    #[serde(rename = "tax_exempt")]
    pub tax_exempt: String,
    #[serde(rename = "tax_ids")]
    pub tax_ids: TaxIds,
    #[serde(rename = "tax_info")]
    pub tax_info: Value,
    #[serde(rename = "tax_info_verification")]
    pub tax_info_verification: Value,
    #[serde(rename = "test_clock")]
    pub test_clock: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceSettings {
    #[serde(rename = "custom_fields")]
    pub custom_fields: Value,
    #[serde(rename = "default_payment_method")]
    pub default_payment_method: Value,
    pub footer: Value,
    #[serde(rename = "rendering_options")]
    pub rendering_options: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sources {
    pub object: String,
    pub data: Vec<Value>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "total_count")]
    pub total_count: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscriptions {
    pub object: String,
    pub data: Vec<Value>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "total_count")]
    pub total_count: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxIds {
    pub object: String,
    pub data: Vec<Value>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "total_count")]
    pub total_count: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub amount: String,
    pub friendly: String,
    pub dummy: bool,
    pub current: bool,
    pub metadata: Metadata2,
    pub ext: String,
    pub perks: Vec<String>,
    pub comped: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata2 {
    #[serde(rename = "type")]
    pub type_field: String,
}

/*
  src/responses/settings.rs
*/
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsResponse {
    pub force: Value,
    pub redirect: Value,
    pub cors: Value,
    pub hsts: Value,
    pub ttl: Value,
}

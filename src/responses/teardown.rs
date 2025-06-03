use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeardownResponse {
    pub msg: String,
    pub ns_domain: String,
    pub instances: Vec<Instance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    #[serde(rename = "type")]
    pub type_field: String,
    pub provider: Option<String>,
    pub domain: String,
    pub location: String,
    pub status: String,
    pub status_color: String,
    pub confirmation: String,
    pub confirmation_color: String,
    pub ip: String,
    pub info: String,
}

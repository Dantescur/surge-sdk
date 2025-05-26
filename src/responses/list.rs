/*
  src/responses/list.rs
*/
use std::fmt;

/*
  src/responses/list.rs
*/
/*
  responses/list.rs
*/
use serde::{Deserialize, Serialize};

use super::CommonMetadata;

pub type ListResponse = Vec<ListResponseElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponseElement {
    pub domain: String,
    pub plan_name: PlanName,
    #[serde(flatten)]
    pub metadata: CommonMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlanName {
    Standard,
}

impl fmt::Display for PlanName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlanName::Standard => write!(f, "Standard"),
        }
    }
}

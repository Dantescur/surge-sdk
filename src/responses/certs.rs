/*
  src/responses/certs.rs
*/
/*
  responses/certs.rs
*/
use serde::{Deserialize, Serialize};

use super::Cert;

#[derive(Debug, Serialize, Deserialize)]
pub struct DCertsResponse {
    pub certs: Vec<Cert>,
}

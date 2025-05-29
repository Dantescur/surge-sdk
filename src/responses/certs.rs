/*
  src/responses/certs.rs
*/
use serde::{Deserialize, Serialize};

use super::Cert;

#[derive(Debug, Serialize, Deserialize)]
pub enum CertResponse<T> {
    Cert(T),
    Certs(Vec<T>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DCertsResponse {
    pub data: CertResponse<Cert>,
}

impl<T> CertResponse<T> {
    pub fn as_slice(&self) -> &[T] {
        match self {
            CertResponse::Cert(val) => std::slice::from_ref(val),
            CertResponse::Certs(vals) => vals,
        }
    }
}

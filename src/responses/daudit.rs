/*
  src/responses/daudit.rs
*/
/*
  responses/daudit.rs
*/
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type DAuditResponse = HashMap<String, DAuditResponseValue>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DAuditResponseValue {
    pub rev: i64,
    pub private_file_list: Vec<Option<serde_json::Value>>,
    pub public_file_count: i64,
    pub public_total_size: i64,
    pub private_file_count: i64,
    pub private_total_size: i64,
    pub manifest: HashMap<String, Manifest>,
    pub cert: Cert,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cert {
    pub subject: Subject,
    pub issuer: Issuer,
    pub subjectaltname: Subjectaltname,
    #[serde(rename = "infoAccess")]
    pub info_access: HashMap<String, Vec<String>>,
    pub modulus: String,
    pub bits: i64,
    pub exponent: Exponent,
    pub pubkey: Pubkey,
    pub valid_from: ValidFrom,
    pub valid_to: ValidTo,
    pub fingerprint: Fingerprint,
    pub fingerprint256: String,
    pub ext_key_usage: Vec<ExtKeyUsage>,
    #[serde(rename = "serialNumber")]
    pub serial_number: SerialNumber,
    pub raw: Pubkey,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Exponent {
    #[serde(rename = "0x10001")]
    The0X10001,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExtKeyUsage {
    #[serde(rename = "1.3.6.1.5.5.7.3.1")]
    The136155731,
    #[serde(rename = "1.3.6.1.5.5.7.3.2")]
    The136155732,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Fingerprint {
    #[serde(rename = "E9:68:90:13:7C:DC:A0:78:C2:9E:E7:15:65:EA:F9:0C:CC:CF:22:40")]
    E96890137CDcA078C29EE71565EaF90CCcCf2240,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Issuer {
    pub c: C,
    pub st: St,
    pub l: L,
    pub o: O,
    pub cn: IssuerCn,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum C {
    #[serde(rename = "GB")]
    Gb,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IssuerCn {
    #[serde(rename = "Sectigo RSA Domain Validation Secure Server CA")]
    SectigoRsaDomainValidationSecureServerCa,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum L {
    Salford,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum O {
    #[serde(rename = "Sectigo Limited")]
    SectigoLimited,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum St {
    #[serde(rename = "Greater Manchester")]
    GreaterManchester,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pubkey {
    #[serde(rename = "type")]
    pub pubkey_type: Type,
    pub data: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    Buffer,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SerialNumber {
    #[serde(rename = "E189924E862D92298F894DFDB7B1084B")]
    E189924E862D92298F894Dfdb7B1084B,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Subject {
    pub cn: SubjectCn,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubjectCn {
    #[serde(rename = "*.surge.sh")]
    SurgeSh,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Subjectaltname {
    #[serde(rename = "DNS:*.surge.sh, DNS:surge.sh")]
    DnsSurgeShDnsSurgeSh,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ValidFrom {
    #[serde(rename = "May  6 00:00:00 2025 GMT")]
    May60000002025Gmt,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ValidTo {
    #[serde(rename = "Jun  4 23:59:59 2026 GMT")]
    Jun42359592026Gmt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub size: i64,
    #[serde(rename = "md5sum")]
    pub md5_sum: String,
    #[serde(rename = "sha256sum")]
    pub sha256_sum: String,
}

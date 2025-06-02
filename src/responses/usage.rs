/*
  src/responses/usage.rs
*/
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageResponse {
    pub normalized_at: String,
    pub version: String,
    pub domain: String,
    pub range: Vec<String>,
    pub traffic: Traffic,
    pub encryption: Encryption,
    pub bandwidth: Bandwidth,
    pub cache: Cache,
    pub source: Source,
    pub device: Device,
    pub os: Os,
    pub browser: Browser,
    pub success: Success,
    pub fail: Fail,
    pub redirect: Redirect,
    pub load: Load,
    pub datacenters: Datacenters,
    pub normalized_at_in_words: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Traffic {
    pub connections: Connections,
    pub visits: Visits,
    pub uniques: Uniques,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connections {
    pub t: i64,
    pub s: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Visits {
    pub t: i64,
    pub s: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uniques {
    pub t: i64,
    pub s: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encryption {
    pub c_e: CE,
    pub c_u: CU,
    pub c_re: CRe,
    pub c_ru: CRu,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CE {
    pub t: i64,
    pub s: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CU {
    pub t: i64,
    pub s: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CRe {
    pub t: i64,
    pub s: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CRu {
    pub t: i64,
    pub s: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bandwidth {
    pub all: All,
    pub body: Body,
    pub headers: Headers,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct All {
    pub t: i64,
    pub s: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub t: i64,
    pub s: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headers {
    pub t: i64,
    pub s: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cache {
    pub hit: Hit,
    pub miss: Miss,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    pub t: i64,
    pub s: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Miss {
    pub t: i64,
    pub s: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    #[serde(rename = "2025-06-02")]
    pub n2025_06_02: Vec<Value>,
    #[serde(rename = "2025-06-01")]
    pub n2025_06_01: Vec<Value>,
    #[serde(rename = "2025-05-31")]
    pub n2025_05_31: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    #[serde(rename = "2025-06-02")]
    pub n2025_06_02: Vec<Value>,
    #[serde(rename = "2025-06-01")]
    pub n2025_06_01: Vec<Value>,
    #[serde(rename = "2025-05-31")]
    pub n2025_05_31: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Os {
    #[serde(rename = "2025-06-02")]
    pub n2025_06_02: Vec<Value>,
    #[serde(rename = "2025-06-01")]
    pub n2025_06_01: Vec<Value>,
    #[serde(rename = "2025-05-31")]
    pub n2025_05_31: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Browser {
    #[serde(rename = "2025-06-02")]
    pub n2025_06_02: Vec<Value>,
    #[serde(rename = "2025-06-01")]
    pub n2025_06_01: Vec<Value>,
    #[serde(rename = "2025-05-31")]
    pub n2025_05_31: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Success {
    #[serde(rename = "2025-06-02")]
    pub n2025_06_02: Vec<Value>,
    #[serde(rename = "2025-06-01")]
    pub n2025_06_01: Vec<Value>,
    #[serde(rename = "2025-05-31")]
    pub n2025_05_31: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fail {
    #[serde(rename = "2025-06-02")]
    pub n2025_06_02: Vec<Value>,
    #[serde(rename = "2025-06-01")]
    pub n2025_06_01: Vec<Value>,
    #[serde(rename = "2025-05-31")]
    pub n2025_05_31: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Redirect {
    #[serde(rename = "2025-06-02")]
    pub n2025_06_02: Vec<Value>,
    #[serde(rename = "2025-06-01")]
    pub n2025_06_01: Vec<Value>,
    #[serde(rename = "2025-05-31")]
    pub n2025_05_31: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Load {
    #[serde(rename = "2025-06-02")]
    pub n2025_06_02: Vec<Value>,
    #[serde(rename = "2025-06-01")]
    pub n2025_06_01: Vec<Value>,
    #[serde(rename = "2025-05-31")]
    pub n2025_05_31: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datacenters {
    #[serde(rename = "syd-01")]
    pub syd_01: Syd01,
    #[serde(rename = "jfk-01")]
    pub jfk_01: Jfk01,
    #[serde(rename = "sfo-16")]
    pub sfo_16: Sfo16,
    #[serde(rename = "sjc-00")]
    pub sjc_00: Sjc00,
    #[serde(rename = "blr-01")]
    pub blr_01: Blr01,
    #[serde(rename = "lhr-01")]
    pub lhr_01: Lhr01,
    #[serde(rename = "sfo-12")]
    pub sfo_12: Sfo12,
    #[serde(rename = "sfo-15")]
    pub sfo_15: Sfo15,
    #[serde(rename = "yyz-06")]
    pub yyz_06: Yyz06,
    #[serde(rename = "nrt-01")]
    pub nrt_01: Nrt01,
    #[serde(rename = "fra-04")]
    pub fra_04: Fra04,
    #[serde(rename = "ams-02")]
    pub ams_02: Ams02,
    #[serde(rename = "yyz-02")]
    pub yyz_02: Yyz02,
    #[serde(rename = "sfo-14")]
    pub sfo_14: Sfo14,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Syd01 {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Jfk01 {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sfo16 {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sjc00 {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blr01 {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lhr01 {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sfo12 {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sfo15 {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Yyz06 {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nrt01 {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fra04 {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ams02 {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Yyz02 {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sfo14 {
    pub t: i64,
    pub s: Vec<i64>,
    pub city: String,
    pub country: String,
}

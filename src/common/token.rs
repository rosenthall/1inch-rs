use serde::{Deserialize};

/// Struct defines TokenInfo object.
/// Contains basic information about specific token
#[derive(Debug, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,

    #[serde(rename = "logoURI")]
    pub logo_uri: String,

    #[serde(rename = "domainVersion")]
    pub domain_version: Option<String>,

    #[serde(rename = "eip2612")]
    pub eip2612: Option<bool>,

    #[serde(rename = "isFoT")]
    pub is_fot: Option<bool>,

    pub tags: Vec<String>,
}

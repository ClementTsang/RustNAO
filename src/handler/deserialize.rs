//! Collection of structs used to deserialize the API JSON results, upon which it is further processed.

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Header {
    pub similarity: String,
    pub thumbnail: String,
    pub index_id: u32,
    pub index_name: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Data {
    #[serde(default)]
    pub ext_urls: Vec<String>,
    pub title: Option<String>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SauceJSON {
    #[serde(default)]
    pub header: Header,
    #[serde(default)]
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct ResultHeader {
    #[serde(default)]
    pub long_limit: String,
    #[serde(default)]
    pub short_limit: String,
    #[serde(default)]
    pub long_remaining: u32,
    #[serde(default)]
    pub short_remaining: u32,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub status: i32,
}

#[derive(Deserialize, Debug)]
pub struct SauceResult {
    pub header: ResultHeader,
    #[serde(default)]
    pub results: Option<Vec<SauceJSON>>,
}

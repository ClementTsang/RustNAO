extern crate serde;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Header {
	pub similarity: String,
	pub thumbnail : String,
	pub index_id: i32,
	pub index_name : String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Data {
	#[serde(default)]
	pub ext_urls: Vec<String>,
}

#[derive(Deserialize, Debug)]
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
	pub long_remaining: i32,
	#[serde(default)]
	pub short_remaining: i32,
	pub status: i32,
}

#[derive(Deserialize, Debug)]
pub struct SauceResult {
	pub header: ResultHeader,
	#[serde(default)]
	pub results: Option<Vec<SauceJSON>>,
}
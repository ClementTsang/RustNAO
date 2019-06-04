extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Header {
	pub similarity: String,
	pub thumbnail : String,
	pub index_id: u32,
	pub index_name : String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AuthorFields {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub da_id : Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub author_name : Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub author_url : Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pixiv_id: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub member_name : Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub member_id : Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bcy_id: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub member_link_id : Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub md_id : Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mu_id : Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mal_id : Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub source : Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub part : Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub anidb_aid : Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pawoo_id : Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pawoo_user_acct : Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pawoo_user_username : Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pawoo_user_display_name : Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub drawr_id : Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub seiga_id : Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nijie_id : Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub konachan_id : Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sankaku_id : Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub danbooru_id : Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub creator : Option<serde_json::Value>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Data {
	#[serde(default)]
	pub ext_urls: Vec<String>,
	pub title: Option<String>,

	#[serde(flatten)]
	pub author_fields: Option<AuthorFields>,
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


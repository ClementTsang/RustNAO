extern crate serde;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Header {
	pub similarity: String,
	pub thumbnail : String,
	pub index_id: u32,
	pub index_name : String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Data {
	#[serde(default)]
	pub ext_urls: Vec<String>,
	pub title: Option<String>,
	pub da_id : Option<u32>,
	pub author_name : Option<String>,
	pub author_url : Option<String>,
	pub pixiv_id: Option<u32>,
	pub member_name : Option<String>,
	pub member_id : Option<u32>,
	pub bcy_id: Option<u32>,
	pub member_link_id : Option<u32>,
	pub md_id : Option<u32>,
	pub mu_id : Option<u32>,
	pub mal_id : Option<u32>,
	pub source : Option<String>,
	pub part : Option<String>,
	pub anidb_aid : Option<u32>,
	pub pawoo_id : Option<u32>,
	pub pawoo_user_acct : Option<String>,
	pub pawoo_user_username : Option<String>,
	pub pawoo_user_display_name : Option<String>,
	pub drawr_id : Option<u32>,
	pub seiga_id : Option<u32>,
	pub nijie_id : Option<u32>,
	pub konachan_id : Option<u32>,
	pub sankaku_id : Option<u32>,
	pub danbooru_id : Option<u32>,
	pub creator : Option<serde_json::Value>,
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


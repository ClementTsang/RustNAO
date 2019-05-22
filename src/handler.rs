#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(while_true)]

extern crate reqwest;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Header {
	similarity: String,
	index_id: i32,
}

#[derive(Deserialize, Debug)]
struct Data {
	ext_urls: Vec<String>,
	#[serde(default)]
	title: String,
	#[serde(default)]
	da_id: u64,
	#[serde(default)]
	author_name: String,
	#[serde(default)]
	author_url: String,
	#[serde(default)]
	pixiv_id: u64,
	#[serde(default)]
	member_name: String,
	#[serde(default)]
	member_id: u64,
}

#[derive(Deserialize, Debug)]
struct Sauce {
	header: Header,
	data: Data,
}

#[derive(Deserialize, Debug)]
struct SauceResult {
	#[serde(default)]
	results_requested: i32,
	#[serde(default)]
	results_returned: i32,
	results: Vec<Sauce>,
}

pub struct Handler<'a> {
	api_key : &'a str,
	num_results : i32,
	db_mask : Vec<i32>,
	db_mask_i : Vec<i32>,
	db : i32,
}

impl Handler<'_> {

	pub fn init(api_key : &str, num_results : i32, db_mask : Vec<i32>, db_mask_i : Vec<i32>, db : i32) -> Handler {
		Handler {
			api_key : api_key,
			num_results : num_results,
			db_mask : db_mask,
			db_mask_i : db_mask_i,
			db : db,
		}
	}

	pub fn get_sauce(&self) -> Result<(), Error> {
		let request_url = format!("https://saucenao.com/search.php?db={db}&output_type={output_type}&testmode={testmode}&numres={numres}&url={url}", db = "999", output_type = "2", testmode = "1", numres = "16", url="http%3A%2F%2Fsaucenao.com%2Fimages%2Fstatic%2Fbanner.gif");
		println!("Request URL: {}", request_url);
		let mut response = reqwest::get(&request_url)?;

		let users: SauceResult = response.json()?;
		println!("{:?}", users);
		Ok(())
	}
}

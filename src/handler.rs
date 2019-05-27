#![crate_name = "rust_nao"]

#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_attributes)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

use serde::{Deserialize, Serialize};
use reqwest::Error;
use std::fmt;
use url::{Url, ParseError};

#[derive(Deserialize, Debug)]
struct Header {
	similarity: String,
	thumbnail : String,
	index_id: i32,
	index_name : String,
}

#[derive(Deserialize, Debug)]
struct Data {
	#[serde(default)]
	ext_urls: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct SauceJSON {
	header: Header,
	data: Data,
}

#[derive(Deserialize, Debug)]
struct ResultHeader {
	long_remaining: i32,
	short_remaining: i32,
}

#[derive(Deserialize, Debug)]
struct SauceResult {
	header: ResultHeader,
	results: Vec<SauceJSON>,
}

#[derive(Serialize)]
pub struct Sauce {
	ext_urls: Vec<String>,
	site: String,
	index: i32,
	similarity: f32,
	thumbnail: String,
	rating: i32,
	author_id: Option<Vec<String>>,
}

impl Sauce {
	fn new() -> Sauce {
		Sauce {
			ext_urls : Vec::new(),
			site: "".to_string(),
			index: -1,
			similarity: 0.0,
			thumbnail : "".to_string(),
			rating: -1,
			author_id: None, 
		}
	}

	fn init(ext_urls : Vec<String>, site : String, index : i32, similarity : f32, thumbnail : String, rating : i32, author_id : Option<Vec<String>>) -> Sauce {
		Sauce {
			ext_urls : ext_urls,
			site : site,
			index : index,
			similarity : similarity,
			thumbnail : thumbnail,
			rating : rating, 
			author_id : author_id,
		}
	}
}

impl fmt::Debug for Sauce {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut result : String = String::new();
		result.push_str("ext_urls: ");
		for i in self.ext_urls.clone() {
			result.push_str(i.as_str());
			result.push_str("  ");
		}
		result.push_str("\nsite: ");
		result.push_str(self.site.as_str());
		result.push_str("\nindex: ");
		result.push_str(self.index.to_string().as_str());
		result.push_str("\nsimilarity: ");
		result.push_str(self.similarity.to_string().as_str());
		result.push_str("\nthumbnail: ");
		result.push_str(self.thumbnail.as_str());
		result.push_str("\nrating: ");
		result.push_str(self.rating.to_string().as_str());
		result.push_str("\nauthor_id: ");
		match self.author_id.clone() {
			Some(author) => {
				for i in author {
					result.push_str(i.as_str());
					result.push_str("  ");
				}
			},
			None =>(),
		}

		write!(f, "{}", result)
	}
}


pub struct Handler<'a> {
	api_key : &'a str,
	output_type : i32,
	testmode : i32,
	num_results : i32,
	db_mask : Vec<i32>,
	db_mask_i : Vec<i32>,
	db : i32,
	short_limit : i32,
	long_limit: i32,
}

impl Handler<'_> {
	/// Creates a new Handler object.  By default, the short limit is set to 30 seconds, and the long limit is set to 24 hours.
	/// ## Arguments
	/// * `api_key` - A string slice holding your api key.
	/// * `testmode` - An i32, either 0 or 1.  Causes each index which has to output at most 1 for testing.
	/// * `db_mask` - A vector of i32 values representing a mask for which database indices you wish to have enabled.
	/// * `db_mask_i` - A vector of i32 values representing a mask for which database indices you wish to have disabled.
	/// * `db` - An i32 value to search for a specific index, or 999 for all.
	/// * `num_results` - An i32 representing the number of results you wish returned.
	/// 
	/// ## Example
	/// ```
	/// use handler::Handler;
	/// ```
	pub fn new(api_key : &str, testmode : i32, db_mask : Vec<i32>, db_mask_i : Vec<i32>, db : i32, num_results : i32) -> Handler {
		assert!(testmode == 1 || testmode == 0, "testmode must be 0 or 1.");

		Handler {
			api_key : api_key,
			output_type : 2,
			testmode : testmode,
			num_results : num_results,
			db_mask : db_mask,
			db_mask_i : db_mask_i,
			db : db,
			short_limit : 30,
			long_limit: 24 * 360,
		}
	}

	/// TODO: May be useless
	pub fn set_short_limit(&mut self, seconds : i32, minutes : i32, hours : i32) {
		self.short_limit = seconds + minutes * 60 + hours * 360;
	}

	/// TODO: May be useless
	pub fn set_long_limit(&mut self, seconds : i32, minutes : i32, hours : i32) {
		self.short_limit = seconds + minutes * 60 + hours * 360;
	}

	fn generate_url(&self, file : &str) -> Result<String, ParseError> {
		let mut request_url = Url::parse("https://saucenao.com/search.php")?;
		request_url.query_pairs_mut().append_pair("api_key", self.api_key);
		request_url.query_pairs_mut().append_pair("output_type", self.output_type.to_string().as_str());
		request_url.query_pairs_mut().append_pair("db", self.db.to_string().as_str());
		request_url.query_pairs_mut().append_pair("testmode", self.testmode.to_string().as_str());
		request_url.query_pairs_mut().append_pair("url", file);

		Ok(request_url.into_string())
	}

	/// Returns a Result of either a vector of Sauce objects, which contain potential sources for the input ``file``, or an Error.
	/// ## Arguments
	/// * ``file`` - A string slice that contains the path to the file you wish to look up.
	/// ## Example
	/// 
	pub fn get_sauce(&self, file : &str) -> Result<Vec<Sauce>, Error> {
		let url_string = self.generate_url(file).unwrap();

		println!("DEBUG: Request URL: {}\n", url_string);
		let mut response = reqwest::get(&url_string)?;

		let returned_sauce: SauceResult = response.json()?;

		// Actual "returned" value:
		let mut ret_sauce : Vec<Sauce> = Vec::new();
		for sauce in returned_sauce.results {
			ret_sauce.push(Sauce::init(
				sauce.data.ext_urls,
				sauce.header.index_name,
				sauce.header.index_id,
				sauce.header.similarity.parse().unwrap(),
				sauce.header.thumbnail.to_string(),
				5,
				None,
			));
		}

		Ok(ret_sauce)
	}

	pub fn get_sauce_as_json(&self, file : &str) -> Result<String, serde_json::Error> {
		let result = String::new();
		let ret_sauce = self.get_sauce(file);

		serde_json::to_string(&ret_sauce.unwrap()) // TODO: Error catching
	}

	// TODO: Async/promise get_sauce?

	// TODO: Remove all insignificant (no ext_urls) searches?

	// TODO: Make one that only returns the first ext_url?

}

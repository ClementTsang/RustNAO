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
	long_limit: String,
	short_limit: String,
	long_remaining: i32,
	short_remaining: i32,
}

#[derive(Deserialize, Debug)]
struct SauceResult {
	header: ResultHeader,
	results: Vec<SauceJSON>,
}

/// A Sauce struct returns one result from a API call made by the Handler
/// ## Examples
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

	/// Returns whether the Sauce struct contains an empty ext_url field.
	/// ## Example
	/// ```
	/// use rust_nao::{Handler, Sauce};
	/// let data = std::fs::read_to_string("config.json").expect("Couldn't read file.");
	/// let api_key = "your_saucenao_api_key";
	/// let file = "https://i.imgur.com/W42kkKS.jpg";
	///	let mut handle = Handler::new(api_key, 0, [].to_vec(), [].to_vec(), 999, 999);
	///	handle.set_min_similarity(45);
	///	let result = handle.get_sauce(file);
	///	if result.is_ok() {
	///		let res : Vec<Sauce> = result.unwrap().into_iter().filter(|sauce| sauce.has_empty_url()).collect();
	///		for i in res {
	///			println!("{:?}", i);
	///		}
	///	}
	///	else {
	///		println!("Failed to make a query."); //TODO: More robust errors
	///	}
	///	
	/// ```
	pub fn has_empty_url(&self) -> bool {
		self.ext_urls.len() <= 0
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

/// A handler struct to make SauceNAO API calls.
/// ## Examples
pub struct Handler {
	api_key : String,
	output_type : i32,
	testmode : i32,
	num_results : i32,
	db_mask : Vec<i32>,
	db_mask_i : Vec<i32>,
	db : i32,
	short_limit : i32,
	long_limit: i32,
	short_left : i32,
	long_left : i32,
	min_similarity : f64,
}

impl Handler {
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
	/// ```
	pub fn new(api_key : &str, testmode : i32, db_mask : Vec<i32>, db_mask_i : Vec<i32>, db : i32, num_results : i32) -> Handler {
		assert!(testmode == 1 || testmode == 0, "testmode must be 0 or 1.");

		Handler {
			api_key : api_key.to_string(),
			output_type : 2,
			testmode : testmode,
			num_results : num_results,
			db_mask : db_mask,
			db_mask_i : db_mask_i,
			db : db,
			short_limit : 12,
			long_limit: 200,
			short_left : 12,
			long_left: 200,
			min_similarity : 0.0,
		}
	}

	/// Sets the minimum similarity threshold for ``get_sauce``.  
	/// ## Arguments
	/// * `similarity` - Represents the minimum similarity threshold you wish to set.  It can be any value that can convert to a f64.  This includes f32s, i16s, i32s, and i8s.
	/// 
	/// ## Example
	/// ```
	/// use rust_nao::Handler;
	/// let mut handle = Handler::new("your_saucenao_api_key", 0, Vec::new(), Vec::new(), 999, 5);
	/// handle.set_min_similarity(50);
	/// ```
	pub fn set_min_similarity<T : Into<f64>>(&mut self, similarity : T) {
		self.min_similarity = similarity.into();
	}

	/// Gets the current short limit as an i32.  By default this is 12.
	/// 
	/// ## Example
	/// ```
	/// ```
	pub fn get_short_limit(&self) -> i32 {
		self.short_limit
	}

	/// Gets the current long limit as an i32.  By default this is 200.
	/// 
	/// ## Example
	/// ```
	/// ```
	pub fn get_long_limit(&self) -> i32 {
		self.long_limit
	}

	/// Gets the current remaining short limit as an i32.
	/// 
	/// ## Example
	/// ```
	/// ```
	pub fn get_current_short_limit(&self) -> i32 {
		self.short_left
	}

	/// Gets the current remaining long limit as an i32.
	/// 
	/// ## Example
	/// ```
	/// ``` 
	pub fn get_current_long_limit(&self) -> i32 {
		self.long_left
	}


	fn generate_url(&self, file : &str) -> Result<String, ParseError> {
		let mut request_url = Url::parse("https://saucenao.com/search.php")?;
		request_url.query_pairs_mut().append_pair("api_key", self.api_key.as_str());
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
	pub fn get_sauce(&mut self, file : &str) -> Result<Vec<Sauce>, Error> {
		let url_string = self.generate_url(file).unwrap();

		println!("DEBUG: Request URL: {}\n", url_string);
		let mut response = reqwest::get(&url_string)?;
		let returned_sauce: SauceResult = response.json()?;

		// Update non-sauce fields
		self.short_left = returned_sauce.header.short_remaining;
		self.long_left = returned_sauce.header.long_remaining;
		self.short_limit = returned_sauce.header.short_limit.parse().unwrap();
		self.long_limit = returned_sauce.header.long_limit.parse().unwrap();

		// Actual "returned" value:
		let mut ret_sauce : Vec<Sauce> = Vec::new();
		for sauce in returned_sauce.results {
			let sauce_min_sim : f64 = sauce.header.similarity.parse().unwrap();
			if sauce_min_sim >= self.min_similarity {
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
		}

		Ok(ret_sauce)
	}

	pub fn get_sauce_as_json(&mut self, file : &str) -> Result<String, serde_json::Error> {
		let result = String::new();
		let ret_sauce = self.get_sauce(file);

		serde_json::to_string_pretty(&ret_sauce.unwrap()) // TODO: Error catching
	}

	// TODO: Async/promise get_sauce?

	// TODO: Organize the interface to look nicer... refactoring pls.

	// TODO: See if you can do anything about the mutability... ugh
}


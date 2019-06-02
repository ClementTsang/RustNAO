extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

mod error;
use error::SauceError;

mod constants;

mod sauce;
pub use sauce::Sauce;

mod deserialize;
use deserialize::SauceResult;

use url::{Url, ParseError};


/// A handler struct to make SauceNAO API calls.
/// ## Examples
/// ```
/// ```
#[derive(Debug, Clone)]
pub struct Handler {
	api_key : String,
	output_type : i32,
	testmode : i32,
	num_results : i32,
	db_mask : Vec<u32>,
	db_mask_i : Vec<u32>,
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
	/// * `db` - An i32 value to search for a specific index, -1 to rely on the bitmasks instead, or 999 for all.
	/// * `num_results` - An i32 representing the number of results you wish returned.
	/// 
	/// ## Example
	/// ```
	/// ```
	pub fn new(api_key : &str, testmode : i32, db_mask : Vec<u32>, db_mask_i : Vec<u32>, db : i32, num_results : i32) -> Handler {
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
	/// * `similarity` - Represents the minimum similarity threshold (in percent) you wish to set.  It can be any value that can convert to a f64.  This includes f32s, i16s, i32s, and i8s.
	/// 
	/// ## Example
	/// ```
	/// use rustnao::Handler;
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

	/// Generates a bitmask from a given vector.
	fn generate_bitmask(&self, mask : Vec<u32>) -> u32 {
		let mut res : u32 = 0;
		for m in mask {
			let mut offset = 0;
			if m >= 18 {
				offset = 1;	// This seems to be some required fix...
			}
			res ^= u32::pow(2, m - offset);
		}
		res
	}

	/// Generates a url from the given image url
	/// 
	fn generate_url(&self, image_url : &str) -> Result<String, ParseError> {
		let mut request_url = Url::parse(constants::API_URL)?;
		request_url.query_pairs_mut().append_pair("api_key", self.api_key.as_str());
		request_url.query_pairs_mut().append_pair("output_type", self.output_type.to_string().as_str());
		if self.db == -1 {
			if self.db_mask.len() > 0 {
				request_url.query_pairs_mut().append_pair("dbmask", self.generate_bitmask(self.db_mask.clone()).to_string().as_str());
			}
			if self.db_mask_i.len() > 0 {
				request_url.query_pairs_mut().append_pair("dbmaski", self.generate_bitmask(self.db_mask_i.clone()).to_string().as_str());
			}
		}
		else {
			request_url.query_pairs_mut().append_pair("db", self.db.to_string().as_str());
		}
		request_url.query_pairs_mut().append_pair("testmode", self.testmode.to_string().as_str());
		request_url.query_pairs_mut().append_pair("numres", self.num_results.to_string().as_str());
		request_url.query_pairs_mut().append_pair("url", image_url);

		Ok(request_url.into_string())
	}

	/// Returns a Result of either a vector of Sauce objects, which contain potential sources for the input ``file``, or a SauceError.
	/// ## Arguments
	/// * ``url`` - A string slice that contains the url of the image you wish to look up.
	/// ## Example
	/// ```
	/// use rustnao::Handler;
	/// let mut handle = Handler::new("your_saucenao_api_key", 0, Vec::new(), Vec::new(), 999, 5);
	/// handle.get_sauce("https://i.imgur.com/W42kkKS.jpg");
	/// ```
	pub fn get_sauce(&mut self, url : &str) -> Result<Vec<Sauce>, SauceError> {
		let url_string = self.generate_url(url)?;
		//println!("{:?}", url_string);
		let returned_sauce: SauceResult = reqwest::get(&url_string)?.json()?;
		let mut ret_sauce : Vec<Sauce> = Vec::new();
		if returned_sauce.header.status >= 0 {
			// Update non-sauce fields
			self.short_left = returned_sauce.header.short_remaining;
			self.long_left = returned_sauce.header.long_remaining;
			self.short_limit = returned_sauce.header.short_limit.parse().unwrap();
			self.long_limit = returned_sauce.header.long_limit.parse().unwrap();

			// Actual "returned" value:

			match returned_sauce.results {
				Some(res) => {
					for sauce in res {
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
				}
				None => (),
			}
			Ok(ret_sauce)
		}
		else {
			Err(SauceError::new(format!("Invalid status code: {}: {}", returned_sauce.header.status, returned_sauce.header.message).as_str()))
		}
		
	}

	/// Returns a string representing a vector of Sauce objects as a serialized JSON, or an error.
	/// ## Arguments
	/// * ``url`` - A string slice that contains the url of the image you wish to look up
	/// ## Example
	/// ```
	/// use rustnao::Handler;
	/// let mut handle = Handler::new("your_saucenao_api_key", 0, Vec::new(), Vec::new(), 999, 5);
	/// handle.get_sauce_as_pretty_json("https://i.imgur.com/W42kkKS.jpg");
	/// ```
	pub fn get_sauce_as_pretty_json(&mut self, url : &str) -> Result<String, SauceError> {
		let ret_sauce = self.get_sauce(url)?;
		Ok(serde_json::to_string_pretty(&ret_sauce)?)
	}

	/// Returns a string representing a vector of Sauce objects as a serialized JSON, or an error.
	/// ## Arguments
	/// * ``url`` - A string slice that contains the url of the image you wish to look up
	/// ## Example
	/// ```
	/// use rustnao::Handler;
	/// let mut handle = Handler::new("your_saucenao_api_key", 0, Vec::new(), Vec::new(), 999, 5);
	/// handle.get_sauce_as_json("https://i.imgur.com/W42kkKS.jpg");
	/// ```
	pub fn get_sauce_as_json(&mut self, url : &str) -> Result<String, SauceError> {
		let ret_sauce = self.get_sauce(url)?;
		Ok(serde_json::to_string(&ret_sauce)?)
	}

	/*
	///
	fn get_sauce_async(&mut self, url : &str) {

	}

	///
	fn get_sauce_as_json_async(&mut self, url : &str) {

	}
	*/

	// TODO: Organize the interface to look nicer... refactoring pls.
}


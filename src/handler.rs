#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(while_true)]

extern crate reqwest;
use reqwest::Error;
use std::fmt;

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
struct SauceJSON {
	header: Header,
	data: Data,
}

#[derive(Deserialize, Debug)]
struct ResultHeader {
	results_requested: i32,
	results_returned: i32,
	long_remaining: i32,
	short_remaining: i32,
}

#[derive(Deserialize, Debug)]
struct SauceResult {
	header: ResultHeader,
	results: Vec<SauceJSON>,
}

pub struct Sauce {
	ext_urls: Vec<String>, // TODO: Might switch this to one string... or make it an option.  Perhaps "getAllURLs"
	site: String,	// TODO: Might switch this to &str also
	index: i32,
	similarity: f32,
	thumbnail: String,
	rating: i32,
	author_id: Option<Vec<String>>,
}

impl Sauce {
	pub fn new(ext_urls : Vec<String>, site : String, index : i32, similarity : f32, thumbnail : String, rating : i32, author_id : Option<Vec<String>>) -> Sauce {
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
			result.push_str(", ");
		}
		result.push_str("\nsite:");
		result.push_str(self.site.as_str());
		result.push_str("\nindex");
		result.push_str(self.index.to_string().as_str());
		result.push_str("\nsimilarity:");
		result.push_str(self.similarity.to_string().as_str());
		result.push_str("\nthumbnail:");
		result.push_str(self.thumbnail.as_str());
		result.push_str("\nrating:");
		result.push_str(self.rating.to_string().as_str());
		result.push_str("\nauthor_id:");
		match self.author_id.clone() {
			Some(author) => {
				for i in author {
					result.push_str(i.as_str());
					result.push_str(", ");
				}
			},
			None =>(),
		}

		write!(f, "{}", result)
	}
}

pub struct Handler<'a> {
	api_key : &'a str,
	num_results : i32,
	db_mask : Vec<i32>,
	db_mask_i : Vec<i32>,
	db : i32,
}

impl Handler<'_> {
	pub fn new(api_key : &str, num_results : i32, db_mask : Vec<i32>, db_mask_i : Vec<i32>, db : i32) -> Handler {
		Handler {
			api_key : api_key,
			num_results : num_results,
			db_mask : db_mask,
			db_mask_i : db_mask_i,
			db : db,
		}
	}

	pub fn get_sauce(&self) -> Result<Vec<Sauce>, Error> {
		let request_url = format!("https://saucenao.com/search.php?db={db}&output_type={output_type}&testmode={testmode}&numres={numres}&url={url}", db = "999", output_type = "2", testmode = "1", numres = "16", url="http%3A%2F%2Fsaucenao.com%2Fimages%2Fstatic%2Fbanner.gif");
		println!("Request URL: {}", request_url);
		let mut response = reqwest::get(&request_url)?;

		let returnedSauce: SauceResult = response.json()?;
		println!("{:?}", returnedSauce);
		println!("-----");

		// Actual "returned" value:
		let mut retSauce : Vec<Sauce> = Vec::new();
		for sauce in returnedSauce.results {
			retSauce.push(Sauce::new(
				sauce.data.ext_urls,
				"SITE".to_string(),
				sauce.header.index_id,
				sauce.header.similarity.parse().unwrap(),
				"THUMBNAIL".to_string(),
				5,
				None,
			));
		}

		Ok(retSauce)
	}
}

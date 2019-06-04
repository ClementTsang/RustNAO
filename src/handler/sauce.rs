extern crate serde;

use serde::Serialize;
use std::fmt;

/// A Sauce struct returns one result from a API call made by the Handler
/// ## Examples
#[derive(Serialize)]
pub struct Sauce {
	pub ext_urls: Vec<String>,
	pub title: Option<String>,
	pub site: String,
	pub index: u32,
	pub index_id: u32,
	pub similarity: f32,
	pub thumbnail: String,
	pub additional_fields: Option<serde_json::Value>,
}

impl Sauce {
	pub fn new() -> Sauce {
		Sauce {
			ext_urls : Vec::new(),
			title: None,
			site: "".to_string(),
			index: 999,
			index_id: 999,
			similarity: 0.0,
			thumbnail : "".to_string(),
			additional_fields: None, 
		}
	}

	pub fn init(ext_urls : Vec<String>, title : Option<String>, site : String, index : u32, index_id : u32, similarity : f32, thumbnail : String, additional_fields : Option<serde_json::Value>) -> Sauce {
		Sauce {
			ext_urls : ext_urls,
			title: title,
			site : site,
			index : index,
			index_id : index_id,
			similarity : similarity,
			thumbnail : thumbnail,
			additional_fields : additional_fields,
		}
	}

	/// Returns whether the Sauce struct contains an empty ext_url field.
	/// ## Example
	/// ```
	/// use rustnao::{Handler, Sauce};
	/// let file = "https://i.imgur.com/W42kkKS.jpg";
	///	let mut handle = Handler::new("your_saucenao_api_key", 0, [].to_vec(), [].to_vec(), 999, 999);
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
			result.push_str(format!("\"{}\"", i.as_str()).as_str());
			result.push_str("  ");
		}
		result.push_str("\nsite: ");
		result.push_str(self.site.as_str());
		match &self.title {
			Some(x) => {
				result.push_str("\ntitle: ");
				result.push_str(x.as_str());
			}
			None => (),
		}
		result.push_str("\nindex: ");
		result.push_str(self.index.to_string().as_str());
		result.push_str("\nindex_id: ");
		result.push_str(self.index_id.to_string().as_str());
		result.push_str("\nsimilarity: ");
		result.push_str(self.similarity.to_string().as_str());
		result.push_str("\nthumbnail: ");
		result.push_str(format!("\"{}\"", self.thumbnail.as_str()).as_str());
		result.push_str("\nauthor_id: ");
		match self.additional_fields.clone() {
			Some(author) => {
				result.push_str(author.to_string().as_str());
			},
			None =>(),
		}

		write!(f, "{}", result)
	}
}

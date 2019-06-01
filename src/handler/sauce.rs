extern crate serde;

use serde::Serialize;
use std::fmt;

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
	pub fn new() -> Sauce {
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

	pub fn init(ext_urls : Vec<String>, site : String, index : i32, similarity : f32, thumbnail : String, rating : i32, author_id : Option<Vec<String>>) -> Sauce {
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

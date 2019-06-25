extern crate rustnao;

use rustnao::{Handler};

const FILE : &str = "https://i.imgur.com/W42kkKS.jpg";
const INVALID_URL : &str = "https://j.jmgur.com";
const INVALID_FILE : &str = "./fake_file.png";

/// Creates a handler for testing purposes
fn create_handler(dbmask : Vec<u32>, dbmaski : Vec<u32>, db : Option<u32>, numres : i32) -> Handler {
	let data = std::fs::read_to_string("config.json");
	if data.is_ok() {
		match data.ok() {
			Some(val) => {
				let json : serde_json::Value = serde_json::from_str(val.as_str()).expect("JSON not well formatted.");
				let api_key = json["api_key"].as_str();

				match api_key {
					Some(key) => Handler::new(key, Some(0), Some(dbmask), Some(dbmaski), db, Some(numres)),
					None => Handler::new("", Some(0), Some(dbmask), Some(dbmaski), db, Some(numres)),
				}
			}
			None => Handler::new("", Some(0), Some(dbmask), Some(dbmaski), db, Some(numres)),
		}
	}
	else {
		return Handler::new("", Some(0), Some(dbmask), Some(dbmaski), db, Some(numres));
	}
}

/// Tests an invalid URL
#[test]
fn test_invalid_url() {
	let handler = create_handler([].to_vec(), [].to_vec(), Some(999), 999);
	let result = handler.get_sauce(INVALID_URL, None, None);
	assert!(result.is_err());
}

/// Tests an invalid URL
#[test]
fn test_invalid_url_json() {
	let handler = create_handler([].to_vec(), [].to_vec(), None, 999);
	let result = handler.get_sauce_as_json(INVALID_URL, None, None);
	assert!(result.is_err());
}

/// Tests an invalid file
#[test]
fn test_invalid_file() {
	let handler = create_handler([].to_vec(), [].to_vec(), Some(999), 999);
	let result = handler.get_sauce(INVALID_FILE, None, None);
	assert!(result.is_err());
}

/// Tests an invalid file
#[test]
fn test_invalid_file_json() {
	let handler = create_handler([].to_vec(), [].to_vec(), Some(999), 999);
	let result = handler.get_sauce(INVALID_FILE, None, None);
	assert!(result.is_err());
}

/// Tests an invalid number of results
#[test]
fn test_invalid_num_results() {
	let handle = create_handler([].to_vec(), [].to_vec(), Some(999), 2);
	let vec = handle.get_sauce(FILE, Some(1000), None);
	assert!(vec.is_err());
}

/// Tests an invalid minimum similarity option (upper)
#[test]
fn test_invalid_min_similarity_upper() {
	let handle = create_handler([].to_vec(), [].to_vec(), Some(999), 2);
	let vec_two = handle.get_sauce(FILE, None, Some(100.1));
	assert!(vec_two.is_err());
}

/// Tests an invalid minimum similarity option (lower)
#[test]
fn test_invalid_min_similarity_lower() {
	let handle = create_handler([].to_vec(), [].to_vec(), Some(999), 2);
	let vec_two = handle.get_sauce(FILE, None, Some(-0.1));
	assert!(vec_two.is_err());
}
extern crate rustnao;

use rustnao::{Handler};

const INVALID_URL : &str = "https://j.jmgur.com";
const INVALID_FILE : &str = "./fake_file.png";

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

#[test]
fn test_invalid_url() {
	let handler = create_handler([].to_vec(), [].to_vec(), Some(999), 999);
	let result = handler.get_sauce(INVALID_URL, None, None);
	assert!(result.is_err());
}

#[test]
fn test_invalid_url_json() {
	let handler = create_handler([].to_vec(), [].to_vec(), None, 999);
	let result = handler.get_sauce_as_json(INVALID_URL, None, None);
	assert!(result.is_err());
}

#[test]
fn test_invalid_file() {
	let handler = create_handler([].to_vec(), [].to_vec(), Some(999), 999);
	let result = handler.get_sauce(INVALID_FILE, None, None);
	assert!(result.is_err());
}

#[test]
fn test_invalid_file_json() {
	let handler = create_handler([].to_vec(), [].to_vec(), Some(999), 999);
	let result = handler.get_sauce(INVALID_FILE, None, None);
	assert!(result.is_err());
}
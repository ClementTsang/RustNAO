extern crate rustnao;

use rustnao::{Handler};

const INVALID_FILE : &str = "https://j.jmgur.com";

fn create_handler(dbmask : Vec<u32>, dbmaski : Vec<u32>, db : i32, numres : i32) -> Handler {
	let data = std::fs::read_to_string("config.json");
	if data.is_ok() {
		match data.ok() {
			Some(val) => {
				let json : serde_json::Value = serde_json::from_str(val.as_str()).expect("JSON not well formatted.");
				let api_key = json["api_key"].as_str();

				match api_key {
					Some(key) => Handler::new(key, 0, dbmask, dbmaski, db, numres),
					None => Handler::new("", 0, dbmask, dbmaski, db, numres),
				}
			}
			None => Handler::new("", 0, dbmask, dbmaski, db, numres),
		}
	}
	else {
		Handler::new("", 0, dbmask, dbmaski, db, numres)
	}
}

#[test]
fn test_invalid_file() {
	let mut handler = create_handler([].to_vec(), [].to_vec(), 999, 999);
	let result = handler.get_sauce(INVALID_FILE);
	assert!(result.is_err());
}

#[test]
fn test_invalid_file_json() {
	let mut handler = create_handler([].to_vec(), [].to_vec(), 999, 999);
	let result = handler.get_sauce_as_json(INVALID_FILE);
	assert!(result.is_err());
}

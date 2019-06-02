extern crate rustnao;

use rustnao::{Handler, Sauce};

const FILE : &str = "https://i.imgur.com/W42kkKS.jpg";

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
fn test_check_handler_creation() {
	create_handler([].to_vec(), [].to_vec(), 999, 999);
}

#[test]
fn test_get_short_and_long_limits() {
	let mut handle = create_handler([].to_vec(), [].to_vec(), 999, 999);
	let short_before = handle.get_short_limit();
	let cur_short_before = handle.get_current_short_limit();
	let long_before = handle.get_long_limit();
	let cur_long_before = handle.get_current_long_limit();
	handle.get_sauce(FILE).unwrap();
	assert!(short_before == handle.get_short_limit());
	assert!(cur_short_before > handle.get_current_short_limit());
	assert!(long_before == handle.get_long_limit());
	assert!(cur_long_before > handle.get_current_long_limit());
}

#[test]
fn test_filter_empty_sauce() {
	let mut handle = create_handler([].to_vec(), [].to_vec(), 999, 999);
	let vec : Vec<Sauce> = handle.get_sauce(FILE).unwrap();
	let only_empty : Vec<Sauce> = vec.into_iter().filter(|sauce| !sauce.has_empty_url()).collect();
	for o in only_empty {
		assert!(o.ext_urls.len() > 0);
	}
}

#[test]
fn test_limiting() {
	let mut handle = create_handler([].to_vec(), [].to_vec(), 999, 2);
	let vec = handle.get_sauce(FILE).unwrap();
	assert!(vec.len() <= 2);
}

#[test]
fn test_db_bit_mask() {
	let mut handle = create_handler([27].to_vec(), [].to_vec(), -1, 999);
	let vec = handle.get_sauce(FILE).unwrap();
	for v in vec {
		assert!(v.index >= 30);
	}
}
#[test]
fn test_db_bit_mask_i() {
	let mut handle = create_handler([].to_vec(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec(), -1, 999);
	let vec = handle.get_sauce(FILE).unwrap();
	for v in vec {
		assert!(v.index >= 11);
	}
}

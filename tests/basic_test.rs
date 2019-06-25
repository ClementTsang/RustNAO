extern crate rustnao;

use rustnao::{Handler, Sauce};

const FILE : &str = "https://i.imgur.com/W42kkKS.jpg";
const LOCAL_FILE: &str = "./tests/test.jpg";

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
fn test_check_handler_creation() {
	create_handler([].to_vec(), [].to_vec(), Some(999), 999);
}

#[test]
fn test_get_short_and_long_limits() {
	let handle = create_handler([].to_vec(), [].to_vec(), Some(999), 999);
	let cur_short_before = handle.get_current_short_limit();
	let cur_long_before = handle.get_current_long_limit();
	let result = handle.get_sauce(FILE, None, None);
	if !result.is_err() {
		assert!(cur_short_before > handle.get_current_short_limit(), format!("{} vs {}", cur_short_before, handle.get_current_short_limit()));
		assert!(cur_long_before > handle.get_current_long_limit(), format!("{} vs {}", cur_long_before, handle.get_current_long_limit()));
	}
}

#[test]
fn test_filter_empty_sauce() {
	let handle = create_handler([].to_vec(), [].to_vec(), Some(999), 999);
	let vec : rustnao::Result<Vec<Sauce>> = handle.get_sauce(FILE, None, None);
	if !vec.is_err() {
		let only_empty : Vec<Sauce> = vec.unwrap().into_iter().filter(|sauce| !sauce.has_empty_url()).collect();
		for o in only_empty {
			assert!(o.ext_urls.len() > 0);
		}
	}
}

#[test]
fn test_local() {
	let handle = create_handler([].to_vec(), [].to_vec(), Some(999), 2);
	let vec = handle.get_sauce_as_json(LOCAL_FILE, None, None);
	match vec {
		Ok(result) => println!("Passed, {}", result),
		Err(result) => println!("Errored out, {}", result),
	}
}

#[test]
fn test_limiting() {
	let handle = create_handler([].to_vec(), [].to_vec(), Some(999), 2);
	let vec = handle.get_sauce(FILE, None, None);
	if !vec.is_err() {
		assert!(vec.unwrap().len() <= 2);
	}
}

#[test]
fn test_db_bit_mask() {
	let handle = create_handler([27].to_vec(), [].to_vec(), None, 999);
	let vec = handle.get_sauce(FILE, None, None);
	if !vec.is_err() {
		for v in vec.unwrap() {
			assert!(v.index >= 27, "saw {}", v.index);
		}
	}
}
#[test]
fn test_db_bit_mask_i() {
	let handle = create_handler([].to_vec(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec(), None, 999);
	let vec = handle.get_sauce(FILE, None, None);
	if !vec.is_err() {
		for v in vec.unwrap() {
			assert!(v.index >= 11);
		}
	}
}

#[test]
fn test_min_similarity_and_num_results() {
	let handle = create_handler([].to_vec(), [].to_vec(), Some(999), 2);
	let vec = handle.get_sauce(LOCAL_FILE, Some(5), Some(50 as f64));
	if !vec.is_err() {
		let res = vec.unwrap();
		assert!(res.len() <= 5);
		for v in res {
			assert!(v.similarity >= 49.0);
		}
	}
}
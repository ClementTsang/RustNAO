//! Example where errors are caught

extern crate rustnao;
use rustnao::{Handler, Result};

fn get_source(file : &str) -> Result<String> {
	let handle = Handler::new("", Some(0), None, None, Some(999), Some(999));
	handle.get_sauce_as_pretty_json(file)
}

fn get_source_string(file : &str) -> String {
	let result = get_source(file);
	match result {
		Ok(res) => res,
		Err(err) => err.to_string(),
	}
}

fn main() {
	let file = "https://i.imgur.com/W42kkKS.jpg";
	let invalid_file = "https://j.jmgur.jpg";
	println!("{}", get_source_string(file));
	println!("{}", get_source_string(invalid_file));
}
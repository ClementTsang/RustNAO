//! Example where errors are caught

extern crate rustnao;
use rustnao::{HandlerBuilder, Result};

fn get_source(file : &str) -> Result<String> {
	let handle = HandlerBuilder::new().api_key("").db(999).num_results(99).build();
	handle.get_sauce_as_pretty_json(file, None, None)
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
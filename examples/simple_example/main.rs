//! A simple example, assuming you had a config.json file that had your api key.

extern crate rustnao;
use rustnao::{HandlerBuilder, Sauce};

fn main() {
	let data = std::fs::read_to_string("config.json").expect("Couldn't read file.");
	let json : serde_json::Value = serde_json::from_str(data.as_str()).expect("JSON not well formatted.");
	let api_key = json["api_key"].as_str();
	let file = "https://i.imgur.com/W42kkKS.jpg";

	match api_key {
		Some(key) => {
			let handle = HandlerBuilder::new().api_key(key).build();
			let result : Vec<Sauce> = handle.get_sauce(file, None, None).unwrap();
			for i in result {
				println!("{:?}", i);
			}
		},
		None => (),
	}
}
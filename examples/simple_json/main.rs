//! Similar to the simple example, except now it's JSON

extern crate rustnao;
use rustnao::{Handler};

fn main() {
	let data = std::fs::read_to_string("config.json").expect("Couldn't read file.");
	let json : serde_json::Value = serde_json::from_str(data.as_str()).expect("JSON not well formatted.");
	let api_key = json["api_key"].as_str();
	let file = "https://i.imgur.com/W42kkKS.jpg";

	match api_key {
		Some(key) => {
			let mut handle = Handler::new(key, 0, [Handler::PIXIV].to_vec(), [].to_vec(), -1, 999);
			let result = handle.get_sauce_as_pretty_json(file).unwrap();
			println!("{}", result);
		},
		None => (),
	}
}
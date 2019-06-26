//! A simple example, assuming you had a config.json file that had your api key.

extern crate rustnao;
use rustnao::{Handler, Sauce, ToJSON};

fn main() {
	let data = std::fs::read_to_string("config.json").expect("Couldn't read file.");
	let json : serde_json::Value = serde_json::from_str(data.as_str()).expect("JSON not well formatted.");
	let api_key = json["api_key"].as_str();
	let file = "https://s5.mangadex.org/data/f2cf04ff9fbd5374c20d1cd5a555efed/x2.png";

	match api_key {
		Some(key) => {
			let handle = Handler::new(key, Some(0), None, None, Some(999), Some(999));
			handle.set_empty_filter(true);
			let result : Vec<Sauce> = handle.get_sauce(file, None, None).unwrap();
			println!("{}", result.to_json_pretty().unwrap());
		},
		None => (),
	}
}
//! Similar to the simple example, except now we only want to get results from Pixiv

extern crate rustnao;
use rustnao::{Handler, Sauce};

fn main() {
	let data = std::fs::read_to_string("config.json").expect("Couldn't read file.");
	let json : serde_json::Value = serde_json::from_str(data.as_str()).expect("JSON not well formatted.");
	let api_key = json["api_key"].as_str();
	let file = "https://i.imgur.com/W42kkKS.jpg";

	match api_key {
		Some(key) => {
			let handle = Handler::new(key, Some(0), Some([Handler::PIXIV].to_vec()), None, None, Some(999));
			let result : Vec<Sauce> = handle.get_sauce(file, None, None).unwrap();
			for i in result {
				println!("{:?}", i);
			}
		},
		None => (),
	}
}
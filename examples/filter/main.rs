//! Now, we wish to filter out our results such that we have a minimum similarity and we reject any empty sources

extern crate rustnao;
use rustnao::{Handler, Sauce};

fn main() {
	let data = std::fs::read_to_string("config.json").expect("Couldn't read file.");
	let json : serde_json::Value = serde_json::from_str(data.as_str()).expect("JSON not well formatted.");
	let api_key = json["api_key"].as_str();
	let file = "https://i.imgur.com/W42kkKS.jpg";

	match api_key {
		Some(key) => {
			let handle = Handler::new(key, Some(0), None, None, Some(999), Some(999));
			handle.set_min_similarity(61.31);
			let result : Vec<Sauce> = handle.get_sauce(file).unwrap().into_iter().filter(|sauce| !sauce.has_empty_url()).collect();  // Remove empty results
			for i in result {
				println!("{:?}", i);
			}
		},
		None => (),
	}
}
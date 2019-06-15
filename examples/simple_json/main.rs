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
			// Specifying our key, testmode set to 0, only want to see Pixiv and Sankaku using a mask, nothing excluded, no one specific source, and 999 results at most
			let handle = Handler::new(key, Some(0), Some([Handler::PIXIV, Handler::SANKAKU_CHANNEL].to_vec()), Some([].to_vec()), None, Some(999));
			let result = handle.get_sauce_as_pretty_json(file).unwrap();
			println!("{}", result);
		},
		None => (),
	}
}
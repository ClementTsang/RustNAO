//! Similar to the simple example, except now it's JSON

extern crate rustnao;
use rustnao::{Handler, HandlerBuilder};

fn main() {
	let data = std::fs::read_to_string("config.json").expect("Couldn't read file.");
	let json: serde_json::Value = serde_json::from_str(data.as_str()).expect("JSON not well formatted.");
	let api_key = json["api_key"].as_str();
	let file = "https://i.imgur.com/W42kkKS.jpg";

	if let Some(key) = api_key {
		// Specifying our key, testmode set to 0, only want to see Pixiv and Sankaku using a mask, nothing excluded, no one specific source, and 999 results at most
		let handle = HandlerBuilder::default()
			.api_key(key)
			.num_results(999)
			.db_mask([Handler::PIXIV, Handler::SANKAKU_CHANNEL].to_vec())
			.build();
		let result = handle.get_sauce_as_pretty_json(file, None, None).unwrap();
		println!("{}", result);
	}
}

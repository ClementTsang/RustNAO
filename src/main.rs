#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(while_true)]

#[macro_use]
extern crate serde;

mod handler;
use handler::Handler;

fn main() {
	let data = std::fs::read_to_string("config.json").expect("Couldn't read file.");
	let json : serde_json::Value = serde_json::from_str(data.as_str()).expect("JSON not well formatted.");
	let api_key = json["api_key"].as_str();

	match api_key {
		Some(key) => {
			let val = Handler::new(key, 0, [].to_vec(), [].to_vec(), 999, 999);
			let result = val.get_sauce("https://cdn.discordapp.com/attachments/329966811057618944/581875763028951074/74728875_p0.jpg");
			if (result.is_ok()) {
				for i in result.unwrap() {
					println!("{:?}", i);
				}
			}
			else {
				println!("Failed to make a query."); //TODO: More robust errors
			}
		},
		None => (),
	}
}

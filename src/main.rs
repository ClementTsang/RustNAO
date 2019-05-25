#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]

#[macro_use]
extern crate serde;
extern crate serde_json;

mod handler;

fn main() {
	let data = std::fs::read_to_string("config.json").expect("Couldn't read file.");
	let json : serde_json::Value = serde_json::from_str(data.as_str()).expect("JSON not well formatted.");
	let api_key = json["api_key"].as_str();

	match api_key {
		Some(key) => {
			let val = handler::Handler::new(key, 999, [].to_vec(), [].to_vec(), 999);
			let result = val.get_sauce();
			if (result.is_ok()) {
				for i in result.unwrap() {
					println!("{:?}", i);
				}
			}
			else {
				println!("Failed to make a query.");
			}
		},
		None => (),
	}
}

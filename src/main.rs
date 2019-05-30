mod lib;
use lib::{Handler, Sauce};

fn main() {
	let data = std::fs::read_to_string("config.json").expect("Couldn't read file.");
	let json : serde_json::Value = serde_json::from_str(data.as_str()).expect("JSON not well formatted.");
	let api_key = json["api_key"].as_str();
	let file = "https://i.imgur.com/W42kkKS.jpg";

	match api_key {
		Some(key) => {
			let mut handle = Handler::new(key, 0, [].to_vec(), [].to_vec(), 999, 999);
			handle.set_min_similarity(45);
			let result = handle.get_sauce(file);
			if result.is_ok() {
				let res : Vec<Sauce> = result.unwrap().into_iter().filter(|sauce| !sauce.has_empty_url()).collect();
				for i in res {
					println!("{:?}", i);
				}
			}
			else {
				println!("Failed to make a query."); //TODO: More robust errors
			}
			println!("------");
			println!("JSON Representation: {}", handle.get_sauce_as_json(file).unwrap());
			println!("------");
			println!("{}", handle.get_current_long_limit());
			println!("{}", handle.get_current_short_limit());
			println!("{}", handle.get_long_limit());
			println!("{}", handle.get_short_limit());
		},
		None => (),
	}
}

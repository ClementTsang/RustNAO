extern crate rustnao;

use rustnao::{Handler};

const INVALID_FILE : &str = "https://j.jmgur.com";

fn create_handler() -> Handler {
	Handler::new("", 0, [].to_vec(), [].to_vec(), 999, 999)
}

#[test]
fn test_invalid_file() {
	let mut handler = create_handler();
	let result = handler.get_sauce(INVALID_FILE);
	assert!(result.is_err());
	println!("{:?}", result);
}

#[test]
fn test_invalid_file_json() {
	let mut handler = create_handler();
	let result = handler.get_sauce_as_json(INVALID_FILE);
	assert!(result.is_err());
	println!("{:?}", result);
}

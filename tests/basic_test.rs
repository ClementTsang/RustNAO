extern crate rustnao;

use rustnao::{Handler, Sauce};

const FILE : &str = "https://i.imgur.com/W42kkKS.jpg";

fn create_handler() -> Handler {
	Handler::new("", 0, [].to_vec(), [].to_vec(), 999, 999)
}

#[test]
fn test_check_handler_creation() {
	create_handler();
}

#[test]
fn test_get_sauce() {
	let mut handle = create_handler();
	let vec = handle.get_sauce(FILE).unwrap();
	for v in vec {
		println!("{:?}", v);
	}
}

#[test]
fn test_get_sauce_as_json() {
	let mut handle = create_handler();
	handle.get_sauce_as_json(FILE).unwrap();
}

#[test]
fn test_get_short_and_long_limits() {
	let mut handle = create_handler();
	handle.get_short_limit();
	handle.get_current_short_limit();
	handle.get_long_limit();
	handle.get_current_long_limit();
	handle.get_sauce(FILE).unwrap();
	handle.get_short_limit();
	handle.get_current_short_limit();
	handle.get_long_limit();
	handle.get_current_long_limit();
}

#[test]
fn test_filter_empty_sauce() {
	let mut handle = create_handler();
	let vec : Vec<Sauce> = handle.get_sauce(FILE).unwrap();
	let _only_empty : Vec<Sauce> = vec.into_iter().filter(|sauce| !sauce.has_empty_url()).collect();
}

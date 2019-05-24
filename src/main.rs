#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]

#[macro_use]
extern crate serde;

mod handler;

fn main() {
	let val = handler::Handler::new("test", 999, [].to_vec(), [].to_vec(), 999);
	let result = val.get_sauce().unwrap();
	for i in result {
		println!("{:?}", i);
	}
}

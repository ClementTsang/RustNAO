#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

#[macro_use]
extern crate serde;

mod handler;

fn main() {
	let val = handler::Handler::init("test", 999, [].to_vec(), [].to_vec(), 999);
	val.get_sauce();
}

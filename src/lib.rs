//! # RustNAO
//!
//! A Rust implementation of a wrapper for the SauceNAO API.
//! 
//! ## Installation
//! Add the following to your ``Cargo.toml`` file:
//! ```toml
//! [dependencies]
//! rustnao = "0.1"
//! ```
//!
//! Then, add the following to your ``main.rs`` file:
//! ```
//! extern crate rustnao;
//! ```
//!
//! ## Examples
//! Here's a simple example:
//! ```no_run
//! extern crate rustnao;
//! use rustnao::{Handler, Sauce};
//! 
//! fn main() {
//! 	let api_key = "your_api_key";
//! 	let file = "https://i.imgur.com/W42kkKS.jpg";
//! 
//! 	// Specifying our key, only want to see Pixiv and Sankaku using a mask, and 15 results at most
//! 	let handle = Handler::new(api_key, None, Some([Handler::PIXIV, Handler::SANKAKU_CHANNEL].to_vec()), None, None, Some(15));
//! 
//!		// Set the minimum similiarity to 45.
//! 	handle.set_min_similarity(45);
//! 
//! 	// Returns a vector of Sauce objects if successful
//! 	let result : Vec<Sauce> = handle.get_sauce(file, None, None).unwrap();
//! 
//! 	// Or perhaps you prefer a JSON output
//! 	let result_json : String = handle.get_sauce_as_pretty_json(file, None, None).unwrap();
//! 
//! 	// Or maybe you wish to only get 5 results with a min similarity of 50.0
//! 	let result_json_filtered : String = handle.get_sauce_as_pretty_json(file, Some(5), Some(50 as f64)).unwrap();
//! }
//! ```


// TODO: further docs on use-case (see sagiri)

#![deny(missing_docs)]

extern crate serde;
extern crate serde_json;
extern crate failure;
extern crate reqwest;
extern crate url;

mod handler;
pub use handler::{
	Handler,
	Sauce,
	Result,
	Error,
	ErrType,
};

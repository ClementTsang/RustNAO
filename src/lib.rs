//! # RustNAO
//!
//! A Rust implementation of a wrapper for the SauceNAO API.
//! 
//! ## Installation
//! 
//! ## Examples
//! Here's a simple example:
//! ```rust
//! extern crate rustnao;
//! use rustnao::{Handler, Sauce};
//! 
//! fn main() {
//! 	let api_key = "your_api_key";
//! 	let file = "https://i.imgur.com/W42kkKS.jpg";
//! 
//! 	// Specifying our key, only want to see Pixiv and Sankaku using a mask, and 15 results at most
//! 	let mut handle = Handler::new(key, None, Some([Handler::PIXIV, Handler::SANKAKU_CHANNEL].to_vec()), None, None, Some(15));
//! 
//!     // Set the minimum similiarity to 45.
//! 	handle.set_min_similarity(45);
//! 
//! 	// Returns a vector of Sauce objects if successful
//! 	let result : Vec<Sauce> = handle.get_sauce(file).unwrap();
//! 
//! 	// Or perhaps you prefer a JSON output
//! 	let result_json : String = handle.get_sauce_as_pretty_json(file).unwrap();
//! }
//! ```

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
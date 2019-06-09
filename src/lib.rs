//! A Rust implementation of a wrapper for the SauceNAO API.

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
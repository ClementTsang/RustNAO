//! # RustNAO
//!
//! A Rust implementation of a wrapper for the SauceNAO API.
//!
//![![crates.io link](https://img.shields.io/crates/v/rustnao.svg)](https://crates.io/crates/rustnao)
//![![Documentation](https://docs.rs/rustnao/badge.svg)](https://docs.rs/rustnao)
//!
//! ## Installation
//!
//! Add the following to your ``Cargo.toml`` file:
//!
//! ```toml
//! [dependencies]
//! rustnao = "0.3.4"
//! ```
//!
//! ## Examples
//!
//! Here's a simple example:
//!
//! ```no_run
//! use rustnao::{Handler, HandlerBuilder, Sauce, Result};
//!
//! fn main() {
//!     let api_key = "your_api_key";
//!     let file = "https://i.imgur.com/W42kkKS.jpg";
//!     
//!     // Specifying our key, test_mode set to 0, only want to see Pixiv and Sankaku using a mask, nothing excluded, no one specific source, and 15 results at most.
//!     let handle = HandlerBuilder::default().api_key(api_key).db_mask([Handler::PIXIV, Handler::SANKAKU_CHANNEL].to_vec()).num_results(15).build();
//!     
//!     // Set the minimum similarity to 45.
//!     handle.set_min_similarity(45);
//!     
//!     // Returns a vector of Sauce objects if successful
//!     let result: Vec<Sauce> = handle.get_sauce(file, None, None).unwrap();
//!     
//!     // Or perhaps you prefer a JSON output
//!     let result_json: String = handle.get_sauce_as_pretty_json(file, None, None).unwrap();
//!     
//!     // Or maybe you wish to only get 5 results with a min similarity of 50.0
//!     let result_json_filtered: String = handle.get_sauce_as_pretty_json(file, Some(5), Some(50 as f64)).unwrap();
//! }
//! ```

// TODO: further docs on use-case (see Sagiri)

#![deny(missing_docs)]

mod handler;
pub use handler::{ErrType, Error, Handler, HandlerBuilder, Result, Sauce, ToJSON};

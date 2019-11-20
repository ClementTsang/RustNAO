//! # RustNAO
//!
//! A Rust implementation of a wrapper for the SauceNAO API.
//!
//![![Build Status](https://travis-ci.com/ClementTsang/RustNAO.svg?token=1wvzVgp94E1TZyPNs8JF&branch=master)](https://travis-ci.com/ClementTsang/RustNAO)
//![![crates.io link](https://img.shields.io/crates/v/rustnao.svg)](https://crates.io/crates/rustnao)
//![![Documentation](https://docs.rs/rustnao/badge.svg)](https://docs.rs/rustnao)
//!
//! ## Installation
//! Add the following to your ``Cargo.toml`` file:
//! ```toml
//! [dependencies]
//! rustnao = "0.3.1"
//! ```
//!
//! ## Examples
//! Here's a simple example:
//! ```no_run
//! use rustnao::{Handler, HandlerBuilder, Sauce, Result};
//!
//! fn main() {
//!     let api_key = "your_api_key";
//!     let file = "https://i.imgur.com/W42kkKS.jpg";
//!
//!     // Specifying our key, only want to see Pixiv and Sankaku using a mask, and 15 results at most
//!     let handle = HandlerBuilder::default().api_key(api_key).db_mask([Handler::PIXIV, Handler::SANKAKU_CHANNEL].to_vec()).num_results(15).build();
//!
//!     // Set the minimum similarity to 45.
//!     handle.set_min_similarity(45);
//!
//!     // Returns a vector of Sauce objects if successful
//!     let result : Result<Vec<Sauce>> = handle.get_sauce(file, None, None);
//!
//!     // Or perhaps you prefer a JSON output
//!     let result_json : Result<String> = handle.get_sauce_as_pretty_json(file, None, None);
//!
//!     // Or maybe you wish to only get 5 results with a min similarity of 50.0
//!     let result_json_filtered : Result<String> = handle.get_sauce_as_pretty_json(file, Some(5), Some(50 as f64));
//!
//!     // Or perhaps you need this as async
//!     async_std::task::block_on(async { async_get_sauce(file, Some(5), Some(50 as f64)).await; });
//! }
//! ```

// TODO: further docs on use-case (see Sagiri)

#![deny(missing_docs)]

extern crate async_std;
extern crate failure;
extern crate serde;
extern crate serde_json;
extern crate surf;
extern crate url;

mod handler;
pub use handler::{ErrType, Error, Handler, HandlerBuilder, Result, Sauce, ToJSON};

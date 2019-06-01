# RustNAO

A Rust implementation of a wrapper for the SauceNAO API.

[![Build Status](https://travis-ci.com/ClementTsang/RustNAO.svg?token=1wvzVgp94E1TZyPNs8JF&branch=master)](https://travis-ci.com/ClementTsang/RustNAO)

## Installation

## Examples
Here's a simple example:
```rust
extern crate rustnao;
use rustnao::{Handler, Sauce};

fn main() {
	let api_key = "your_api_key";
	let file = "https://i.imgur.com/W42kkKS.jpg";

	let mut handle = Handler::new(key, 0, [].to_vec(), [].to_vec(), 999, 999);
	handle.set_min_similarity(45);
	let result = handle.get_sauce(file);
	if result.is_ok() {
		let res : Vec<Sauce> = result.unwrap();
	}
	else {
		println!("Failed to make a query.");
	}
}
```

## Documentation
Documentation can be found here.

## Development
Interested in helping?  Found a problem/bug?  Let me know!

## Thanks
I was mostly inspired by [Sagiri](https://github.com/ClarityCafe/Sagiri), so a huge shoutout to that project.  Furthermore, thanks to [SauceNAO](https://saucenao.com/) which provides this amazing functionality for free.
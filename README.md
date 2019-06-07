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

	// Specifying our key, testmode set to 0, only want to see Pixiv and Sankaku using a mask, nothing excluded, no one specific source, and 999 results at most
	let mut handle = Handler::new(key, Some(0), Some([Handler::PIXIV, Handler::SANKAKU_CHANNEL].to_vec()), Some([].to_vec()), None, Some(999));
	handle.set_min_similarity(45);

	// Returns a vector of Sauce objects if successful
	let result : Vec<Sauce> = handle.get_sauce(file).unwrap();

	// Or perhaps you prefer a JSON output
	let result_json : String = handle.get_sauce_as_pretty_json(file).unwrap();
}
```

## Documentation
Documentation can be found [here]().

## Development
Interested in helping?  Found a problem/bug?  Let me know!

## Thanks
I was mostly inspired by [Sagiri](https://github.com/ClarityCafe/Sagiri), so a huge shoutout to that project.  Furthermore, thanks to [SauceNAO](https://saucenao.com/) which provides this amazing functionality for free.
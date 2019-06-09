# RustNAO

A Rust implementation of a wrapper for the SauceNAO API.

[![Build Status](https://travis-ci.com/ClementTsang/RustNAO.svg?token=1wvzVgp94E1TZyPNs8JF&branch=master)](https://travis-ci.com/ClementTsang/RustNAO) [![crates.io link](https://img.shields.io/crates/v/rustnao.svg)](https://crates.io/crates/rustnao)

## Installation
Add the following to your ``Cargo.toml`` file:
```
[dependencies]
rustnao = "0.1"
```

Then, add the following to your ``main.rs`` file:
```
extern crate rustnao;
```

## Examples
Here's a simple example:
```rust
extern crate rustnao;
use rustnao::{Handler, Sauce};

fn main() {
	let api_key = "your_api_key";
	let file = "https://i.imgur.com/W42kkKS.jpg";

	// Specifying our key, testmode set to 0, only want to see Pixiv and Sankaku using a mask, nothing excluded, no one specific source, and 15 results at most
	let mut handle = Handler::new(api_key, Some(0), Some([Handler::PIXIV, Handler::SANKAKU_CHANNEL].to_vec()), None, None, Some(15));

	// Set the minimum similiarity to 45.
	handle.set_min_similarity(45);

	// Returns a vector of Sauce objects if successful
	let result : Vec<Sauce> = handle.get_sauce(file).unwrap();

	// Or perhaps you prefer a JSON output
	let result_json : String = handle.get_sauce_as_pretty_json(file).unwrap();
}
```

See more examples [here](./examples/).

## Documentation
Further documentation can be found [here](https://docs.rs/rustnao/).  You can also see SauceNAO's API documentation [here](https://saucenao.com/user.php?page=search-api).

## Development
Interested in helping?  Found a problem/bug?  Let me know!

## Thanks
I was inspired by [Sagiri](https://github.com/ClarityCafe/Sagiri), so a huge shoutout to that project.  Furthermore, thanks to [SauceNAO](https://saucenao.com/) which provides this amazing functionality for free.

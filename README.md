# RustNAO

A Rust implementation of a wrapper for the SauceNAO API.

[![crates.io link](https://img.shields.io/crates/v/rustnao.svg)](https://crates.io/crates/rustnao)
[![Documentation](https://docs.rs/rustnao/badge.svg)](https://docs.rs/rustnao)

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
rustnao = "0.3.2"
```

## Examples

Here's a simple example:

```rust
use rustnao::{HandlerBuilder, Sauce};

fn main() {
 let api_key = "your_api_key";
 let file = "https://i.imgur.com/W42kkKS.jpg";

 // Specifying our key, test_mode set to 0, only want to see Pixiv and Sankaku using a mask, nothing excluded, no one specific source, and 15 results at most
 let handle = HandlerBuilder::default().api_key(api_key).db_mask([Handler::PIXIV, Handler::SANKAKU_CHANNEL].to_vec()).num_results(15).build();

 // Set the minimum similarity to 45.
 handle.set_min_similarity(45);

 // Returns a vector of Sauce objects if successful
 let result: Vec<Sauce> = handle.get_sauce(file, None, None).unwrap();

 // Or perhaps you prefer a JSON output
 let result_json: String = handle.get_sauce_as_pretty_json(file, None, None).unwrap();

 // Or maybe you wish to only get 5 results with a min similarity of 50.0
 let result_json_filtered: String = handle.get_sauce_as_pretty_json(file, Some(5), Some(50 as f64)).unwrap();
}
```

See more examples [here](./examples/).

## Documentation

Further documentation can be found [here](https://docs.rs/rustnao/). You can also see SauceNAO's API documentation [here](https://saucenao.com/user.php?page=search-api).

## Development

Interested in helping? Found a problem/bug? Let me know!

## Thanks/Credits

I was inspired by [Sagiri](https://github.com/ClarityCafe/Sagiri), so I have to give a shoutout that project. Furthermore, thanks to [SauceNAO](https://saucenao.com/) which provides this amazing functionality for free.

Also, credit to Pixiv user [リン ☆ ユウ＠1 日目 西れ 44b](https://www.pixiv.net/member.php?id=4754550) for [this image](https://www.pixiv.net/member_illust.php?mode=medium&illust_id=61477678) I used frequently for examples and tests. I couldn't seem to find the image I used for local testing (SauceNAO failed me), if anyone knows let me know so I can credit them.

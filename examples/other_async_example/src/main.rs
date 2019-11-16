//! Similar to the simple example, except now it's async (using async-std)

extern crate rustnao;
use async_std::task;
use rustnao::{Error, Handler, HandlerBuilder};

fn main() {
	let handle = HandlerBuilder::default()
		.api_key("key")
		.num_results(999)
		.db_mask([Handler::PIXIV, Handler::SANKAKU_CHANNEL].to_vec())
		.build();

	let file = "https://i.imgur.com/W42kkKS.jpg";

	task::block_on(async {
		let result = handle.async_get_sauce_as_pretty_json(file, None, None);
		let blocking_result = handle.get_sauce_as_pretty_json(file, None, None).unwrap();
		println!("Blocking result... {}", blocking_result);
		println!("Result... {}", result.await.unwrap());
	})
}

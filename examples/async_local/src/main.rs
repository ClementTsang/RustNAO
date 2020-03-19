//! Similar to the simple example, except now it's async (using async-std)

extern crate rustnao;
use async_std::task;
use rustnao::{Handler, HandlerBuilder};

fn main() {
	let handle = HandlerBuilder::default()
		.api_key("key")
		.num_results(999)
		.db_mask([Handler::PIXIV, Handler::SANKAKU_CHANNEL].to_vec())
		.build();

	task::block_on(async {
		let file = "./test.jpg";
		let result = handle.async_get_sauce_as_pretty_json(file, None, None);
		println!("File Result... {}", result.await.unwrap());
	});
}

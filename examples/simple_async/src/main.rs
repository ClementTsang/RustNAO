//! Similar to the simple example, except now it's async

extern crate rustnao;
use rustnao::{Error, Handler, HandlerBuilder};

#[tokio::main]
async fn main() -> Result<(), Error> {
	let handle = HandlerBuilder::default()
		.api_key("key")
		.num_results(999)
		.db_mask([Handler::PIXIV, Handler::SANKAKU_CHANNEL].to_vec())
		.build();

	let file = "https://i.imgur.com/W42kkKS.jpg";

	println!("This is before code-wise...");

	let result = handle.async_get_sauce_as_pretty_json(file, None, None);
	println!("Blocking... {}", result.await?);

	println!("This is after code-wise...");

	Ok(())
}

//! Handler module of rustnao.  The handler for the SauceNAO API calls.

mod error;
pub use error::{ErrType, Error, Result};

mod constants;

mod sauce;
pub use sauce::Sauce;

mod deserialize;
use deserialize::SauceResult;

use std::cell::Cell;
use url::Url;

/// A builder to create a Handler for RustNAO usage.
/// ## Example
/// ```
/// use rustnao::HandlerBuilder;
/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db(999).build();
/// ```
#[derive(Default, Debug, Clone)]
pub struct HandlerBuilder {
	api_key: Option<String>,
	testmode: Option<bool>,
	db_mask: Option<Vec<u32>>,
	db_mask_i: Option<Vec<u32>>,
	db: Option<u32>,
	num_results: Option<u32>,
	min_similarity: Option<f64>,
	empty_filter_enabled: Option<bool>,
}

impl HandlerBuilder {
	/// Sets the API key used for searches for the Handler.  If this is not set then a blank API key is used, instead of your personal one.
	///
	/// ### Arguments
	/// * api_key - A string reference representing your API key.
	///
	/// ### Examples
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().api_key("your_api_key").build();
	/// ```
	pub fn api_key(&mut self, api_key: &str) -> &mut HandlerBuilder {
		self.api_key = Some(api_key.to_string());
		self
	}

	/// Sets whether testmode should be enabled on searches for the Handler.  If this is on, then each index will output at most one result.  If this is unset then it is set to off by default.
	///
	/// ### Arguments
	/// * testmode - A boolean representing whether you want testmode to be set to on (true) or off (false).
	///
	/// ### Examples
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().testmode(true).build();
	/// ```
	pub fn testmode(&mut self, testmode: bool) -> &mut HandlerBuilder {
		self.testmode = Some(testmode);
		self
	}

	/// Sets which database indices you want included on search for the Handler.  If both db and db_mask are not set, then every index is checked (db_mask_i will still apply).
	///
	/// ### Arguments
	/// * db_mask - A vector of u32s representing the database indices you wish to have included in your search.
	///
	/// ### Examples
	/// ```
	/// use rustnao::{Handler, HandlerBuilder};
	/// let handle = HandlerBuilder::default().db_mask([1, 2, Handler::PIXIV].to_vec()).build();
	/// ```
	pub fn db_mask(&mut self, db_mask: Vec<u32>) -> &mut HandlerBuilder {
		self.db_mask = Some(db_mask);
		self
	}

	/// Sets which database indices you want excluded on search for the Handler.
	///
	/// ### Arguments
	/// * db_mask_i - A vector of u32s representing the database indices you wish to have excluded in your search.
	///
	/// ### Examples
	/// ```
	/// use rustnao::{Handler, HandlerBuilder};
	/// let handle = HandlerBuilder::default().db_mask_i([1, 2, Handler::PIXIV].to_vec()).build();
	/// ```
	pub fn db_mask_i(&mut self, db_mask_i: Vec<u32>) -> &mut HandlerBuilder {
		self.db_mask_i = Some(db_mask_i);
		self
	}

	/// Sets a database index to be searched for the Handler.  If both db and db_mask are not set, then every index is checked (db_mask_i will still apply).
	///
	/// ### Arguments
	/// * db - A u32 representing which database index you want included.  Set it to 999 to include every index.
	///
	/// ### Examples
	/// ```
	/// use rustnao::{Handler, HandlerBuilder};
	/// let handle = HandlerBuilder::default().db(Handler::PIXIV).build();
	/// ```
	pub fn db(&mut self, db: u32) -> &mut HandlerBuilder {
		self.db = Some(db);
		self
	}

	/// Sets the maximum number of results you want returned on search for the Handler.  You can change this number per-search.  If this is not set, by default this is set to return at most 999 results.
	///
	/// ### Arguments
	/// * num_results - A u32 value representing how many results you want returned.
	///
	/// ### Examples
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().num_results(10).build();
	/// ```
	pub fn num_results(&mut self, num_results: u32) -> &mut HandlerBuilder {
		self.num_results = Some(num_results);
		self
	}

	/// Sets he minimum similarity for results by default for the Handler.  You can change this number per-search.  If this is not set, by default it is 0.0 (no minimum similarity).
	///
	/// ### Arguments
	/// * min_similarity : A number that can be cast into a f64 representing the minimum similarity (in percent) of a result to be returned.
	///
	/// ### Examples
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().min_similarity(50.5).build();
	/// ```
	pub fn min_similarity<T: Into<f64>>(&mut self, min_similarity: T) -> &mut HandlerBuilder {
		self.min_similarity = Some(min_similarity.into());
		self
	}

	/// Sets whether to enable an empty filter by default for the Handler.  If this is not set, by default it is false.
	///
	/// ### Arguments
	/// * empty_filter_enabled : A boolean representing whether you want empty URL search results to be filtered out by default.
	///
	/// ### Examples
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().empty_filter_enabled(true).build();
	/// ```
	pub fn empty_filter_enabled(&mut self, empty_filter_enabled: bool) -> &mut HandlerBuilder {
		self.empty_filter_enabled = Some(empty_filter_enabled);
		self
	}

	/// Builds the HandlerBuilder, returning a Handler that can be used to search.
	///
	/// ### Examples
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().api_key("your_api_key").db(999).num_results(50).build();
	/// ```
	pub fn build(&self) -> Handler {
		let mut api_key = "";
		match &self.api_key {
			Some(x) => api_key = x.as_str(),
			None => (),
		}

		let mut testmode = None;
		if let Some(x) = self.testmode {
			testmode = if x { Some(1) } else { Some(0) };
		}

		let mut num_results = None;
		if let Some(x) = self.num_results {
			num_results = Some(x as u32);
		}

		let result = Handler::new(api_key, testmode, self.db_mask.clone(), self.db_mask_i.clone(), self.db, num_results);
		if let Some(x) = self.min_similarity {
			result.set_min_similarity(x);
		}

		if let Some(x) = self.empty_filter_enabled {
			result.set_empty_filter(x);
		}

		result
	}
}

/// A handler struct to make SauceNAO API calls with.
///
/// ## Example
/// ```
/// use rustnao::HandlerBuilder;
/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db(999).build();
/// handle.get_sauce("https://i.imgur.com/W42kkKS.jpg", None, None);
/// ```
#[derive(Debug, Clone)]
pub struct Handler {
	api_key: String,
	output_type: i32,
	testmode: Option<u32>,
	db_mask: Option<Vec<u32>>,
	db_mask_i: Option<Vec<u32>>,
	db: Option<u32>,
	num_results: Option<u32>,
	short_limit: Cell<u32>,
	long_limit: Cell<u32>,
	short_left: Cell<u32>,
	long_left: Cell<u32>,
	min_similarity: Cell<f64>,
	empty_filter_enabled: Cell<bool>,
}

impl Handler {
	/// Associated index for H-Magazines.
	pub const H_MAGAZINES: u32 = constants::H_MAGAZINES.index;
	/// Associated index for H-Game CG.
	pub const H_GAME_CG: u32 = constants::H_GAME_CG.index;
	/// Associated index for DoujinshiDB.
	pub const DOUJINSHI_DB: u32 = constants::DOUJINSHI_DB.index;
	/// Associated index for Pixiv.
	pub const PIXIV: u32 = constants::PIXIV.index;
	/// Associated index for Nico Nico Seiga.
	pub const NICO_NICO_SEIGA: u32 = constants::NICO_NICO_SEIGA.index;
	/// Associated index for Danbooru.
	pub const DANBOORU: u32 = constants::DANBOORU.index;
	/// Associated index for drawr Images.
	pub const DRAWR: u32 = constants::DRAWR.index;
	/// Associated index for Nijie Images.
	pub const NIJIE: u32 = constants::NIJIE.index;
	/// Associated index for Yand.ere.
	pub const YANDE_RE: u32 = constants::YANDE_RE.index;
	/// Associated index for Shutterstock.
	pub const SHUTTERSTOCK: u32 = constants::SHUTTERSTOCK.index;
	/// Associated index for Fakku.
	pub const FAKKU: u32 = constants::FAKKU.index;
	/// Associated index for H-Misc.
	pub const H_MISC: u32 = constants::H_MISC.index;
	/// Associated index for 2D-Market.
	pub const TWO_D_MARKET: u32 = constants::TWO_D_MARKET.index;
	/// Associated index for MediBang.
	pub const MEDIBANG: u32 = constants::MEDIBANG.index;
	/// Associated index for Anime.
	pub const ANIME: u32 = constants::ANIME.index;
	/// Associated index for H-Anime.
	pub const H_ANIME: u32 = constants::H_ANIME.index;
	/// Associated index for Movies.
	pub const MOVIES: u32 = constants::MOVIES.index;
	/// Associated index for Shows.
	pub const SHOWS: u32 = constants::SHOWS.index;
	/// Associated index for Gelbooru.
	pub const GELBOORU: u32 = constants::GELBOORU.index;
	/// Associated index for Konachan.
	pub const KONACHAN: u32 = constants::KONACHAN.index;
	/// Associated index for Sankaku Channel.
	pub const SANKAKU_CHANNEL: u32 = constants::SANKAKU_CHANNEL.index;
	/// Associated index for Anime-Pictures.net.
	pub const ANIME_PICTURES_NET: u32 = constants::ANIME_PICTURES_NET.index;
	/// Associated index for e621.net.
	pub const E621_NET: u32 = constants::E621_NET.index;
	/// Associated index for Idol Complex.
	pub const IDOL_COMPLEX: u32 = constants::IDOL_COMPLEX.index;
	/// Associated index for bcy.net Illust.
	pub const BCY_NET_ILLUST: u32 = constants::BCY_NET_ILLUST.index;
	/// Associated index for bcy.net Cosplay.
	pub const BCY_NET_COSPLAY: u32 = constants::BCY_NET_COSPLAY.index;
	/// Associated index for PortalGraphics.net.
	pub const PORTALGRAPHICS_NET: u32 = constants::PORTALGRAPHICS_NET.index;
	/// Associated index for deviantArt.
	pub const DEVIANTART: u32 = constants::DEVIANTART.index;
	/// Associated index for Pawoo.net.
	pub const PAWOO_NET: u32 = constants::PAWOO_NET.index;
	/// Associated index for Madokami.
	pub const MADOKAMI: u32 = constants::MADOKAMI.index;
	/// Associated index for Mangadex.
	pub const MANGADEX: u32 = constants::MANGADEX.index;

	/// Grabs the appropriate Source data given an index
	fn get_source(&self, index: u32) -> Option<constants::Source<'_>> {
		let mut result: Option<constants::Source<'_>> = None;
		for src in constants::LIST_OF_SOURCES.iter() {
			if src.index == index {
				result = Some(src.clone());
			}
		}
		result
	}

	// TODO: Test bitmask further!
	/// Generates a bitmask from a given vector.
	fn generate_bitmask(&self, mask: Vec<u32>) -> u32 {
		let mut res: u32 = 0;
		for m in mask {
			let offset = if m >= 18 {
				1 // TODO: This seems to be some required fix... look into!
			} else {
				0
			};
			res ^= u32::pow(2, m - offset);
		}
		res
	}

	/// Generates a url from the given image url
	fn generate_url(&self, image_path: &str, num_results: Option<u32>) -> Result<String> {
		let mut request_url = Url::parse(constants::API_URL)?;
		request_url.query_pairs_mut().append_pair("api_key", self.api_key.as_str());
		request_url
			.query_pairs_mut()
			.append_pair("output_type", self.output_type.to_string().as_str());

		if let Some(val) = self.db {
			request_url.query_pairs_mut().append_pair("db", val.to_string().as_str());
		}

		if let Some(val) = &self.db_mask {
			if !val.is_empty() {
				request_url
					.query_pairs_mut()
					.append_pair("dbmask", self.generate_bitmask(val.clone()).to_string().as_str());
			}
		}
		if let Some(val) = &self.db_mask_i {
			if !val.is_empty() {
				request_url
					.query_pairs_mut()
					.append_pair("dbmaski", self.generate_bitmask(val.clone()).to_string().as_str());
			}
		}

		match self.testmode {
			Some(val) => {
				request_url.query_pairs_mut().append_pair("testmode", val.to_string().as_str());
			}
			None => {
				request_url.query_pairs_mut().append_pair("testmode", "0");
			}
		}

		match num_results {
			Some(results) => {
				request_url.query_pairs_mut().append_pair("numres", results.to_string().as_str());
			}
			None => match self.num_results {
				Some(val) => {
					request_url.query_pairs_mut().append_pair("numres", val.to_string().as_str());
				}
				None => {
					request_url.query_pairs_mut().append_pair("numres", "999");
				}
			},
		}
		if image_path.starts_with("https://") || image_path.starts_with("http://") {
			// Link
			request_url.query_pairs_mut().append_pair("url", image_path);
		}

		Ok(request_url.into_string())
	}

	fn new(
		api_key: &str, testmode: Option<u32>, db_mask: Option<Vec<u32>>, db_mask_i: Option<Vec<u32>>, db: Option<u32>, num_results: Option<u32>,
	) -> Handler {
		Handler {
			api_key: api_key.to_string(),
			output_type: 2, // This is set to 2 by default, as we need a JSON reply
			testmode,
			db_mask,
			db_mask_i,
			db,
			num_results,
			short_limit: Cell::new(12),
			long_limit: Cell::new(200),
			short_left: Cell::new(12),
			long_left: Cell::new(200),
			min_similarity: Cell::new(0.0),
			empty_filter_enabled: Cell::new(false),
		}
	}

	/// Sets the minimum similarity threshold for ``get_sauce``.  By default this is 0.0.
	/// ## Arguments
	/// * `min_similarity` - Represents the minimum similarity threshold (in percent) you wish to set.  It can be any value that can convert to a f64.  This includes f32s, i16s, i32s, and i8s.
	///
	/// ## Example
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db(999).build();
	/// handle.set_min_similarity(50);
	/// ```
	pub fn set_min_similarity<T: Into<f64>>(&self, min_similarity: T) {
		self.min_similarity.set(min_similarity.into());
	}

	/// Sets the whether empty URL results should be automatically filtered for ``get_sauce``.  
	/// ## Arguments
	/// * `enabled` - Represents whether filter should be enabled or not.  By default, this is disabled.
	///
	/// ## Example
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db(999).build();
	/// handle.set_empty_filter(true);
	/// ```
	pub fn set_empty_filter(&self, enabled: bool) {
		self.empty_filter_enabled.set(enabled);
	}

	/// Gets the current short limit as an i32.  By default this is 12.
	///
	/// ## Example
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db(999).build();
	/// println!("{}", handle.get_short_limit());
	/// ```
	pub fn get_short_limit(&self) -> u32 {
		self.short_limit.get()
	}

	/// Gets the current long limit as an i32.  By default this is 200.
	///
	/// ## Example
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db(999).build();
	/// println!("{}", handle.get_long_limit());
	/// ```
	pub fn get_long_limit(&self) -> u32 {
		self.long_limit.get()
	}

	/// Gets the current remaining short limit as an i32.
	///
	/// ## Example
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db(999).build();
	/// println!("{}", handle.get_current_short_limit());
	/// ```
	pub fn get_current_short_limit(&self) -> u32 {
		self.short_left.get()
	}

	/// Gets the current remaining long limit as an i32.
	///
	/// ## Example
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db(999).build();
	/// println!("{}", handle.get_current_long_limit());
	/// ```
	pub fn get_current_long_limit(&self) -> u32 {
		self.long_left.get()
	}

	fn is_valid_min_sim(&self, min_similarity: Option<f64>) -> bool {
		if let Some(min_similarity) = min_similarity {
			if min_similarity > 100.0 || min_similarity < 0.0 {
				return false;
			}
		}

		true
	}

	fn is_valid_num_res(&self, num_results: Option<u32>) -> bool {
		if let Some(num_results) = num_results {
			if num_results > 999 {
				return false;
			}
		}

		true
	}

	fn process_results(&self, returned_sauce: SauceResult, min_similarity: Option<f64>) -> Result<Vec<Sauce>> {
		let mut ret_sauce: Vec<Sauce> = Vec::new();

		if returned_sauce.header.status >= 0 {
			// Update non-sauce fields
			self.short_left.set(returned_sauce.header.short_remaining);
			self.long_left.set(returned_sauce.header.long_remaining);
			self.short_limit.set(returned_sauce.header.short_limit.parse()?);
			self.long_limit.set(returned_sauce.header.long_limit.parse()?);

			// Actual "returned" value:
			if let Some(res) = returned_sauce.results {
				let actual_min_sim: f64;
				match min_similarity {
					Some(min_sim) => actual_min_sim = min_sim,
					None => actual_min_sim = self.min_similarity.get(),
				}
				for sauce in res {
					let sauce_min_sim: f64 = sauce.header.similarity.parse()?;
					if (sauce_min_sim >= actual_min_sim)
						&& ((self.empty_filter_enabled.get() && !sauce.data.ext_urls.is_empty()) || !self.empty_filter_enabled.get())
					{
						let actual_index: u32 = sauce.header.index_name.split(':').collect::<Vec<&str>>()[0]
							.to_string()
							.split('#')
							.collect::<Vec<&str>>()[1]
							.to_string()
							.parse::<u32>()?;
						let source: Option<constants::Source> = self.get_source(actual_index);

						match source {
							Some(src) => {
								ret_sauce.push(sauce::new_sauce(
									sauce.data.ext_urls,
									sauce.data.title,
									src.name.to_string(),
									actual_index,
									sauce.header.index_id,
									sauce.header.similarity.parse().unwrap(),
									sauce.header.thumbnail.to_string(),
									match serde_json::to_value(&sauce.data.additional_fields) {
										Ok(x) => Some(x),
										Err(_x) => None,
									},
								));
							}
							None => {
								ret_sauce.push(sauce::new_sauce(
									sauce.data.ext_urls,
									sauce.data.title,
									sauce.header.index_name,
									actual_index,
									sauce.header.index_id,
									sauce.header.similarity.parse().unwrap(),
									sauce.header.thumbnail.to_string(),
									None,
								));
							}
						}
					}
				}
			}
			Ok(ret_sauce)
		} else {
			Err(Error::invalid_code(returned_sauce.header.status, returned_sauce.header.message))
		}
	}

	/// Returns a Result of either a vector of Sauce objects, which contain potential sources for the input file, or a SauceError.
	/// ## Arguments
	/// * ``image_path`` - A string slice that contains the url of the image you wish to look up.
	/// * ``num_results`` - An Option containing a u32 to specify the number of results you wish to get for this specific search.  If this is None, it will default to whatever was originally set in the Handler when it was initalized.  This can be at most 999.
	/// * ``min_similarity`` - An Option containing a f64 to specify the minimum similarity you wish to meet for a result to show up for this specific search.  If this is None, it will default to whatever was originally set in the Handler when it was initalized.
	///
	/// ## Example
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db(999).build();
	/// handle.get_sauce("./tests/test.jpg", None, None);
	/// ```
	///
	/// ## Errors
	/// If there was a problem forming a URL, reading a file, making a request, or parsing the returned JSON, an error will be returned.
	/// Furthermore, if you pass a link in which SauceNAO returns an error code, an error containing the code and message will be returned.
	pub fn get_sauce(&self, image_path: &str, num_results: Option<u32>, min_similarity: Option<f64>) -> Result<Vec<Sauce>> {
		// This is essentially just a blocking version of the async call... thank you, code reuse

		// TODO: Considering phasing this out, and also may need async versions of other helper functions... probably not though
		async_std::task::block_on(async { self.async_get_sauce(image_path, num_results, min_similarity).await })
	}

	/// Returns a string representing a vector of Sauce objects as a serialized JSON, or an error.  Otherwise identical to ``get_sauce(...)``
	/// ## Arguments
	/// * ``image_path`` - A string slice that contains the url of the image you wish to look up.
	/// * ``num_results`` - An Option containing a u32 to specify the number of results you wish to get for this specific search.  If this is None, it will default to whatever was originally set in the Handler when it was initialized.
	/// * ``min_similarity`` - An Option containing a f64 to specify the minimum similarity you wish to meet for a result to show up for this specific search.  If this is None, it will default to whatever was originally set in the Handler when it was initialized.
	///
	/// ## Example
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db(999).build();
	/// handle.get_sauce_as_pretty_json("https://i.imgur.com/W42kkKS.jpg", None, None);
	/// ```
	///
	/// ## Errors
	/// If there was a problem forming a URL, reading a file, making a request, or parsing the returned JSON, an error will be returned.
	/// Furthermore, if you pass a link in which SauceNAO returns an error code, an error containing the code and message will be returned.
	pub fn get_sauce_as_pretty_json(&self, image_path: &str, num_results: Option<u32>, min_similarity: Option<f64>) -> Result<String> {
		let ret_sauce = self.get_sauce(image_path, num_results, min_similarity)?;
		Ok(serde_json::to_string_pretty(&ret_sauce)?)
	}

	/// Returns a string representing a vector of Sauce objects as a serialized JSON, or an error.
	/// ## Arguments
	/// * ``image_path`` - A string slice that contains the url of the image you wish to look up.
	/// * ``num_results`` - An Option containing a u32 to specify the number of results you wish to get for this specific search.  If this is None, it will default to whatever was originally set in the Handler when it was initialized.
	/// * ``min_similarity`` - An Option containing a f64 to specify the minimum similarity you wish to meet for a result to show up for this specific search.  If this is None, it will default to whatever was originally set in the Handler when it was initialized.
	///
	/// ## Example
	/// ```
	/// use rustnao::HandlerBuilder;
	/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db(999).build();
	/// handle.get_sauce_as_json("https://i.imgur.com/W42kkKS.jpg", None, None);
	/// ```
	///
	/// ## Errors
	/// If there was a problem forming a URL, reading a file, making a request, or parsing the returned JSON, an error will be returned.
	/// Furthermore, if you pass a link in which SauceNAO returns an error code, an error containing the code and message will be returned.
	pub fn get_sauce_as_json(&self, image_path: &str, num_results: Option<u32>, min_similarity: Option<f64>) -> Result<String> {
		let ret_sauce = self.get_sauce(image_path, num_results, min_similarity)?;
		Ok(serde_json::to_string(&ret_sauce)?)
	}

	/// Asynchronously returns a Result of either a vector of Sauce objects, which contain potential sources for the input path, or a SauceError.
	/// ## Arguments
	/// * ``image_path`` - A string slice that contains the url of the image you wish to look up.
	/// * ``num_results`` - An Option containing a u32 to specify the number of results you wish to get for this specific search.  If this is None, it will default to whatever was originally set in the Handler when it was initalized.  This can be at most 999.
	/// * ``min_similarity`` - An Option containing a f64 to specify the minimum similarity you wish to meet for a result to show up for this specific search.  If this is None, it will default to whatever was originally set in the Handler when it was initalized.
	///
	/// ## Errors
	/// If there was a problem forming a URL, reading a file, making a request, or parsing the returned JSON, an error will be returned.
	/// Furthermore, if you pass a link in which SauceNAO returns an error code, an error containing the code and message will be returned.
	pub async fn async_get_sauce(&self, image_path: &str, num_results: Option<u32>, min_similarity: Option<f64>) -> Result<Vec<Sauce>> {
		// Check passed in values first to see if they're valid!

		if !self.is_valid_min_sim(min_similarity) {
			return Err(Error::invalid_parameter(
				"min_similarity must be less 100.0 and greater than 0.0.".to_string(),
			));
		} else if !self.is_valid_num_res(num_results) {
			return Err(Error::invalid_parameter("num_results must be less than 999.".to_string()));
		}

		let url_string = self.generate_url(image_path, num_results)?;

		let returned_sauce: SauceResult = if !(image_path.starts_with("https://") || image_path.starts_with("http://")) {
			let mime = mime_guess::from_path(&image_path).first_or_octet_stream();

			let image = std::fs::read(image_path)?;
			let data = serde_json::json!({ "path": image });
			let bd = surf::post(&url_string).body_json(&data)?.set_mime(mime);

			println!("BD: {:?}", bd);

			bd.await?.body_json().await?
		} else {
			surf::post(&url_string).await?.body_json().await?
		};

		self.process_results(returned_sauce, min_similarity)
	}

	/// Asynchronously returns a string representing a vector of Sauce objects as a serialized JSON, or an error.  Otherwise identical to ``async_get_sauce(...)``
	/// ## Arguments
	/// * ``image_path`` - A string slice that contains the url of the image you wish to look up.
	/// * ``num_results`` - An Option containing a u32 to specify the number of results you wish to get for this specific search.  If this is None, it will default to whatever was originally set in the Handler when it was initialized.
	/// * ``min_similarity`` - An Option containing a f64 to specify the minimum similarity you wish to meet for a result to show up for this specific search.  If this is None, it will default to whatever was originally set in the Handler when it was initialized.
	///
	/// ## Errors
	/// If there was a problem forming a URL, reading a file, making a request, or parsing the returned JSON, an error will be returned.
	/// Furthermore, if you pass a link in which SauceNAO returns an error code, an error containing the code and message will be returned.
	pub async fn async_get_sauce_as_json(&self, image_path: &str, num_results: Option<u32>, min_similarity: Option<f64>) -> Result<String> {
		let ret_sauce = self.async_get_sauce(image_path, num_results, min_similarity).await?;
		Ok(serde_json::to_string(&ret_sauce)?)
	}

	/// Asynchronously returns a string representing a vector of Sauce objects as a serialized JSON, or an error.  Otherwise identical to ``async_get_sauce(...)``
	/// ## Arguments
	/// * ``image_path`` - A string slice that contains the url of the image you wish to look up.
	/// * ``num_results`` - An Option containing a u32 to specify the number of results you wish to get for this specific search.  If this is None, it will default to whatever was originally set in the Handler when it was initialized.
	/// * ``min_similarity`` - An Option containing a f64 to specify the minimum similarity you wish to meet for a result to show up for this specific search.  If this is None, it will default to whatever was originally set in the Handler when it was initialized.
	///
	/// ## Errors
	/// If there was a problem forming a URL, reading a file, making a request, or parsing the returned JSON, an error will be returned.
	/// Furthermore, if you pass a link in which SauceNAO returns an error code, an error containing the code and message will be returned.
	pub async fn async_get_sauce_as_pretty_json(&self, image_path: &str, num_results: Option<u32>, min_similarity: Option<f64>) -> Result<String> {
		let ret_sauce = self.async_get_sauce(image_path, num_results, min_similarity).await?;
		Ok(serde_json::to_string_pretty(&ret_sauce)?)
	}
}

/// A trait to convert to JSON and pretty JSON strings.
/// ### Example
/// Implementing for a Sauce vector into a pretty JSON string:
/// ```
/// use rustnao::{HandlerBuilder, ToJSON};
/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db(999).build();
/// let result = handle.get_sauce("./tests/test.jpg", None, None);
/// if result.is_ok() {
/// 	result.unwrap().to_json_pretty();
/// }
/// ```
pub trait ToJSON {
	/// Converts to a Result containing a JSON string.
	/// ### Example
	/// ```
	/// use rustnao::{HandlerBuilder, ToJSON};
	/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db(999).build();
	/// let result = handle.get_sauce("./tests/test.jpg", None, None);
	/// if result.is_ok() {
	/// 	result.unwrap().to_json();
	/// }
	/// ```
	/// ### Errors
	/// There may be a problem converting the object to a JSON string, so this will throw an Error if that is encountered.
	fn to_json(&self) -> Result<String>;

	/// Converts to a Result containing a pretty JSON string.
	/// ### Example
	/// ```
	/// use rustnao::{HandlerBuilder, ToJSON};
	/// let handle = HandlerBuilder::default().api_key("your_api_key").num_results(999).db(999).build();
	/// let result = handle.get_sauce("./tests/test.jpg", None, None);
	/// if result.is_ok() {
	/// 	result.unwrap().to_json_pretty();
	/// }
	/// ```
	/// ### Errors
	/// There may be a problem converting the object to a JSON string, so this will throw an Error if that is encountered.
	fn to_json_pretty(&self) -> Result<String>;
}

impl ToJSON for Vec<Sauce> {
	/// Converts a Sauce vector into a pretty JSON string.
	fn to_json_pretty(&self) -> Result<String> {
		Ok(serde_json::to_string_pretty(self)?)
	}

	/// Converts a Sauce vector into a JSON string.
	fn to_json(&self) -> Result<String> {
		Ok(serde_json::to_string(self)?)
	}
}

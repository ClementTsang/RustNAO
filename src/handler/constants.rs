//! ## A list of constants used by the RustNAO library
//! Constants are pulled from here: https://saucenao.com/status.html

pub const API_URL : &str = "https://saucenao.com/search.php";

pub struct Source<'a> {
	pub index : u32,
	name : &'a str,
	author_fields : Option<&'a[&'a str]>,
	rating_regex : Option<&'a str>,
}

pub const H_MAGAZINES : Source<'static> = Source {
	index: 0,
	name: "H-Magazines",
	author_fields: None,
	rating_regex: Some(""),
};

pub const H_GAME_CG : Source<'static> = Source {
	index: 2,
	name: "H-Game CG",
	author_fields: None,
	rating_regex: Some(""),
};

pub const DOUJINSHI_DB : Source<'static> = Source {
	index: 3,
	name: "DoujinshiDB",
	author_fields: None,
	rating_regex: Some(""),
};

pub const PIXIV : Source<'static> = Source {
	index: 5,
	name: "Pixiv",
	author_fields: Some(&["pixiv_id", "member_name", "member_id"]),
	rating_regex: Some(""),
};
//! ## A list of constants used by the RustNAO library
//! Constants are pulled from here: https://saucenao.com/status.html

pub const API_URL : &str = "https://saucenao.com/search.php";

#[derive(Clone)]
pub struct Source<'a> {
	pub index : u32,
	pub name : &'a str,
	pub author_fields : Option<&'a[&'a str]>,
	pub rating_regex : Option<&'a str>,
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

pub const NICO_NICO_SEIGA : Source<'static> = Source {
	index: 8,
	name: "Nico Nico Seiga",
	author_fields: Some(&["seiga_id", "member_name", "member_id"]),
	rating_regex: Some(""),
};

pub const DANBOORU : Source<'static> = Source {
	index: 9,
	name: "Danbooru",
	author_fields: Some(&["danbooru_id"]),
	rating_regex: Some(""),
};


pub const DRAWR : Source<'static> = Source {
	index: 10,
	name: "drawr Images",
	author_fields: Some(&["drawr_id", "member_name", "member_id"]),
	rating_regex: Some(""),
};

pub const NIJIE : Source<'static> = Source {
	index: 11,
	name: "Nijie Images",
	author_fields: Some(&["nijie_id", "member_name", "member_id"]),
	rating_regex: Some(""),
};

pub const YANDE_RE : Source<'static> = Source {
	index: 12,
	name: "Yande.re",
	author_fields: None,
	rating_regex: Some(""),
};

pub const SHUTTERSTOCK : Source<'static> = Source {
	index: 15,
	name: "Shutterstock",
	author_fields: None,
	rating_regex: Some(""),
};

pub const FAKKU : Source<'static> = Source {
	index: 16,
	name: "FAKKU",
	author_fields: None,
	rating_regex: Some(""),
};

pub const H_MISC : Source<'static> = Source {
	index: 18,
	name: "H-Misc",
	author_fields: None,
	rating_regex: Some(""),
};

pub const TWO_D_MARKET : Source<'static> = Source {
	index: 19,
	name: "2D-Market",
	author_fields: None,
	rating_regex: Some(""),
};

pub const MEDIBANG : Source<'static> = Source {
	index: 20,
	name: "MediBang",
	author_fields: None,
	rating_regex: Some(""),
};

pub const ANIME : Source<'static> = Source {
	index: 21,
	name: "Anime",
	author_fields: Some(&["anidb_aid", "source", "part"]),
	rating_regex: Some(""),
};

pub const H_ANIME : Source<'static> = Source {
	index: 22,
	name: "H-Anime ",
	author_fields: None,
	rating_regex: Some(""),
};

pub const MOVIES : Source<'static> = Source {
	index: 23,
	name: "Movies",
	author_fields: None,
	rating_regex: Some(""),
};

pub const SHOWS : Source<'static> = Source {
	index: 24,
	name: "Shows",
	author_fields: None,
	rating_regex: Some(""),
};

pub const GELBOORU : Source<'static> = Source {
	index: 25,
	name: "Gelbooru",
	author_fields: None,
	rating_regex: Some(""),
};

pub const KONACHAN : Source<'static> = Source {
	index: 26,
	name: "Konachan",
	author_fields: None,
	rating_regex: Some(""),
};

pub const SANKAKU_CHANNEL : Source<'static> = Source {
	index: 27,
	name: "Sankaku Channel",
	author_fields: Some(&["konachan_id", "sankaku_id", "source"]),
	rating_regex: Some(""),
};

pub const ANIME_PICTURES_NET : Source<'static> = Source {
	index: 28,
	name: "Anime-Pictures.net",
	author_fields: None,
	rating_regex: Some(""),
};

pub const E621_NET : Source<'static> = Source {
	index: 29,
	name: "e621.net",
	author_fields: None,
	rating_regex: Some(""),
};

pub const IDOL_COMPLEX : Source<'static> = Source {
	index: 30,
	name: "Idol Complex",
	author_fields: None,
	rating_regex: Some(""),
};

pub const BCY_NET_ILLUST : Source<'static> = Source {
	index: 31,
	name: "bcy.net Illust",
	author_fields: Some(&["bcy_id", "member_name", "member_id", "member_link_id"]),
	rating_regex: None,
};

pub const BCY_NET_COSPLAY : Source<'static> = Source {
	index: 32,
	name: "bcy.net Cosplay",
	author_fields: None,
	rating_regex: Some(""),
};

pub const PORTALGRAPHICS_NET : Source<'static> = Source {
	index: 33,
	name: "PortalGraphics.net",
	author_fields: None,
	rating_regex: Some(""),
};

pub const DEVIANTART : Source<'static> = Source {
	index: 34,
	name: "deviantArt",
	author_fields: Some(&["da_id", "author_name", "author_url"]),
	rating_regex: Some(""),
};

pub const PAWOO_NET : Source<'static> = Source {
	index: 35,
	name: "Pawoo.net",
	author_fields: Some(&["pawoo_id", "pawoo_user_acct", "pawoo_user_username", "pawoo_user_display_name"]),
	rating_regex: Some(""),
};

pub const MADOKAMI : Source<'static> = Source {
	index: 36,
	name: "Madokami",
	author_fields: Some(&["mu_id", "source", "part"]),
	rating_regex: Some(""),
};

pub const MANGADEX : Source<'static> = Source {
	index: 37,
	name: "MangaDex",
	author_fields: Some(&["md_id", "mu_id", "mal_id", "source", "part"]),
	rating_regex: Some(""),
};

pub const LIST_OF_SOURCES : [Source; 31] = [
	H_MAGAZINES, H_GAME_CG, DOUJINSHI_DB, PIXIV, NICO_NICO_SEIGA, DANBOORU,
	DRAWR, NIJIE, YANDE_RE, SHUTTERSTOCK, FAKKU, H_MISC, TWO_D_MARKET, MEDIBANG,
	ANIME, H_ANIME, MOVIES, SHOWS, GELBOORU, KONACHAN, SANKAKU_CHANNEL, ANIME_PICTURES_NET,
	E621_NET, IDOL_COMPLEX, BCY_NET_ILLUST, BCY_NET_COSPLAY, PORTALGRAPHICS_NET, 
	DEVIANTART, PAWOO_NET, MADOKAMI, MANGADEX
];
//! A list of constants used by the RustNAO library.
//! Constants are pulled from here: https://saucenao.com/status.html.

pub const API_URL : &str = "https://saucenao.com/search.php";

#[derive(Clone)]
pub struct Source<'a> {
	pub index : u32,
	pub name : &'a str,
	pub rating_regex : Option<&'a str>,
}

pub const H_MAGAZINES : Source<'static> = Source {
	index : 0,
	name : "H-Magazines",
	rating_regex : Some(""),
};

pub const H_GAME_CG : Source<'static> = Source {
	index : 2,
	name : "H-Game CG",
	rating_regex : Some(""),
};

pub const DOUJINSHI_DB : Source<'static> = Source {
	index : 3,
	name : "DoujinshiDB",
	rating_regex : Some(""),
};

pub const PIXIV : Source<'static> = Source {
	index : 5,
	name : "Pixiv",
	rating_regex : Some(""),
};

pub const NICO_NICO_SEIGA : Source<'static> = Source {
	index : 8,
	name : "Nico Nico Seiga",
	rating_regex : Some(""),
};

pub const DANBOORU : Source<'static> = Source {
	index : 9,
	name : "Danbooru",
	rating_regex : Some(""),
};

pub const DRAWR : Source<'static> = Source {
	index : 10,
	name : "drawr Images",
	rating_regex : Some(""),
};

pub const NIJIE : Source<'static> = Source {
	index : 11,
	name : "Nijie Images",
	rating_regex : Some(""),
};

pub const YANDE_RE : Source<'static> = Source {
	index : 12,
	name : "Yande.re",
	rating_regex : Some(""),
};

pub const SHUTTERSTOCK : Source<'static> = Source {
	index : 15,
	name : "Shutterstock",
	rating_regex : Some(""),
};

pub const FAKKU : Source<'static> = Source {
	index : 16,
	name : "FAKKU",
	rating_regex : Some(""),
};

pub const H_MISC : Source<'static> = Source {
	index : 18,
	name : "H-Misc",
	rating_regex : Some(""),
};

pub const TWO_D_MARKET : Source<'static> = Source {
	index : 19,
	name : "2D-Market",
	rating_regex : Some(""),
};

pub const MEDIBANG : Source<'static> = Source {
	index : 20,
	name : "MediBang",
	rating_regex : Some(""),
};

pub const ANIME : Source<'static> = Source {
	index : 21,
	name : "Anime",
	rating_regex : Some(""),
};

pub const H_ANIME : Source<'static> = Source {
	index : 22,
	name : "H-Anime ",
	rating_regex : Some(""),
};

pub const MOVIES : Source<'static> = Source {
	index : 23,
	name : "Movies",
	rating_regex : Some(""),
};

pub const SHOWS : Source<'static> = Source {
	index : 24,
	name : "Shows",
	rating_regex : Some(""),
};

pub const GELBOORU : Source<'static> = Source {
	index : 25,
	name : "Gelbooru",
	rating_regex : Some(""),
};

pub const KONACHAN : Source<'static> = Source {
	index : 26,
	name : "Konachan",
	rating_regex : Some(""),
};

pub const SANKAKU_CHANNEL : Source<'static> = Source {
	index : 27,
	name : "Sankaku Channel",
	rating_regex : Some(""),
};

pub const ANIME_PICTURES_NET : Source<'static> = Source {
	index : 28,
	name : "Anime-Pictures.net",
	rating_regex : Some(""),
};

pub const E621_NET : Source<'static> = Source {
	index : 29,
	name : "e621.net",
	rating_regex : Some(""),
};

pub const IDOL_COMPLEX : Source<'static> = Source {
	index : 30,
	name : "Idol Complex",
	rating_regex : Some(""),
};

pub const BCY_NET_ILLUST : Source<'static> = Source {
	index : 31,
	name : "bcy.net Illust",
	rating_regex : None,
};

pub const BCY_NET_COSPLAY : Source<'static> = Source {
	index : 32,
	name : "bcy.net Cosplay",
	rating_regex : Some(""),
};

pub const PORTALGRAPHICS_NET : Source<'static> = Source {
	index : 33,
	name : "PortalGraphics.net",
	rating_regex : Some(""),
};

pub const DEVIANTART : Source<'static> = Source {
	index : 34,
	name : "deviantArt",
	rating_regex : Some(""),
};

pub const PAWOO_NET : Source<'static> = Source {
	index : 35,
	name : "Pawoo.net",
	rating_regex : Some(""),
};

pub const MADOKAMI : Source<'static> = Source {
	index : 36,
	name : "Madokami",
	rating_regex : Some(""),
};

pub const MANGADEX : Source<'static> = Source {
	index : 37,
	name : "MangaDex",
	rating_regex : Some(""),
};

pub const LIST_OF_SOURCES : [Source; 31] = [
	H_MAGAZINES,
	H_GAME_CG,
	DOUJINSHI_DB,
	PIXIV,
	NICO_NICO_SEIGA,
	DANBOORU,
	DRAWR,
	NIJIE,
	YANDE_RE,
	SHUTTERSTOCK,
	FAKKU,
	H_MISC,
	TWO_D_MARKET,
	MEDIBANG,
	ANIME,
	H_ANIME,
	MOVIES,
	SHOWS,
	GELBOORU,
	KONACHAN,
	SANKAKU_CHANNEL,
	ANIME_PICTURES_NET,
	E621_NET,
	IDOL_COMPLEX,
	BCY_NET_ILLUST,
	BCY_NET_COSPLAY,
	PORTALGRAPHICS_NET,
	DEVIANTART,
	PAWOO_NET,
	MADOKAMI,
	MANGADEX,
];

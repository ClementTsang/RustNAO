//! A list of constants used by the RustNAO library.
//! Constants are pulled from here: https://saucenao.com/status.html.

pub const API_URL: &str = "https://saucenao.com/search.php";

#[derive(Clone)]
pub struct Source<'a> {
	pub index: u32,
	pub name: &'a str,
}

pub const H_MAGAZINES: Source<'static> = Source {
	index: 0,
	name: "H-Magazines",
};

pub const H_GAME_CG: Source<'static> = Source { index: 2, name: "H-Game CG" };

pub const DOUJINSHI_DB: Source<'static> = Source {
	index: 3,
	name: "DoujinshiDB",
};

pub const PIXIV: Source<'static> = Source { index: 5, name: "Pixiv" };

pub const NICO_NICO_SEIGA: Source<'static> = Source {
	index: 8,
	name: "Nico Nico Seiga",
};

pub const DANBOORU: Source<'static> = Source { index: 9, name: "Danbooru" };

pub const DRAWR: Source<'static> = Source {
	index: 10,
	name: "drawr Images",
};

pub const NIJIE: Source<'static> = Source {
	index: 11,
	name: "Nijie Images",
};

pub const YANDE_RE: Source<'static> = Source { index: 12, name: "Yande.re" };

pub const SHUTTERSTOCK: Source<'static> = Source {
	index: 15,
	name: "Shutterstock",
};

pub const FAKKU: Source<'static> = Source { index: 16, name: "FAKKU" };

pub const H_MISC: Source<'static> = Source { index: 18, name: "H-Misc" };

pub const TWO_D_MARKET: Source<'static> = Source {
	index: 19,
	name: "2D-Market",
};

pub const MEDIBANG: Source<'static> = Source { index: 20, name: "MediBang" };

pub const ANIME: Source<'static> = Source { index: 21, name: "Anime" };

pub const H_ANIME: Source<'static> = Source { index: 22, name: "H-Anime " };

pub const MOVIES: Source<'static> = Source { index: 23, name: "Movies" };

pub const SHOWS: Source<'static> = Source { index: 24, name: "Shows" };

pub const GELBOORU: Source<'static> = Source { index: 25, name: "Gelbooru" };

pub const KONACHAN: Source<'static> = Source { index: 26, name: "Konachan" };

pub const SANKAKU_CHANNEL: Source<'static> = Source {
	index: 27,
	name: "Sankaku Channel",
};

pub const ANIME_PICTURES_NET: Source<'static> = Source {
	index: 28,
	name: "Anime-Pictures.net",
};

pub const E621_NET: Source<'static> = Source { index: 29, name: "e621.net" };

pub const IDOL_COMPLEX: Source<'static> = Source {
	index: 30,
	name: "Idol Complex",
};

pub const BCY_NET_ILLUST: Source<'static> = Source {
	index: 31,
	name: "bcy.net Illust",
};

pub const BCY_NET_COSPLAY: Source<'static> = Source {
	index: 32,
	name: "bcy.net Cosplay",
};

pub const PORTALGRAPHICS_NET: Source<'static> = Source {
	index: 33,
	name: "PortalGraphics.net",
};

pub const DEVIANTART: Source<'static> = Source {
	index: 34,
	name: "deviantArt",
};

pub const PAWOO_NET: Source<'static> = Source {
	index: 35,
	name: "Pawoo.net",
};

pub const MADOKAMI: Source<'static> = Source { index: 36, name: "Madokami" };

pub const MANGADEX: Source<'static> = Source { index: 37, name: "MangaDex" };

pub const LIST_OF_SOURCES: [Source; 31] = [
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

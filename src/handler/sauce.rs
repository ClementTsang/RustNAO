use serde::Serialize;
use std::fmt;

/// A Sauce struct contains one result from a API call made by the Handler.  
/// ## Example
/// ```
/// use rustnao::{Sauce, HandlerBuilder};
/// let mut handle = HandlerBuilder::default().api_key("your_api_key").build();
/// let result : rustnao::Result<Vec<Sauce>> = handle.get_sauce("https://i.pximg.net/img-master/img/2019/02/10/03/11/39/73095123_p0_master1200.jpg", None, None);
/// ```
#[derive(Serialize)]
pub struct Sauce {
    /// A Vec of Strings representing the external URLs for the image
    pub ext_urls: Vec<String>,
    /// An optional String to represent the title of the image
    pub title: Option<String>,
    /// A string to represent the site the image is from
    pub site: String,
    /// The official index on SauceNAO for the index the site corresponds to
    pub index: u32,
    /// The index returned by the SauceNAO API.  Usually this is equal to the ``index`` but sometimes it is different (see Sankaku, for example)
    pub index_id: u32,
    /// The similarity the image has with the guess
    pub similarity: f32,
    /// A string representing the URL of the thumbnail
    pub thumbnail: String,
    /// Any additional fields that are specific to the source
    pub additional_fields: Option<serde_json::Value>,
}

// TODO: Consider making the sauce object a builder...

/// Creates a new Sauce object.
#[allow(clippy::too_many_arguments)]
pub(in crate::handler) fn new_sauce(
    ext_urls: Vec<String>, title: Option<String>, site: String, index: u32, index_id: u32,
    similarity: f32, thumbnail: String, additional_fields: Option<serde_json::Value>,
) -> Sauce {
    Sauce {
        ext_urls,
        title,
        site,
        index,
        index_id,
        similarity,
        thumbnail,
        additional_fields,
    }
}

impl Sauce {
    /// Returns whether the Sauce struct contains an empty ext_url field.
    /// ## Example
    /// ```
    /// use rustnao::{HandlerBuilder, Sauce};
    /// let file = "https://i.imgur.com/W42kkKS.jpg";
    ///    let mut handle = HandlerBuilder::default().api_key("your_api_key").build();
    ///    handle.set_min_similarity(45);
    ///    let result = handle.get_sauce(file, None, None);
    ///    if result.is_ok() {
    ///        let res : Vec<Sauce> = result.unwrap().into_iter().filter(|sauce| sauce.has_empty_url()).collect();
    ///        for i in res {
    ///            println!("{:?}", i);
    ///        }
    ///    }
    ///    else {
    ///        println!("Failed to make a query.");
    ///    }
    ///
    /// ```
    pub fn has_empty_url(&self) -> bool {
        self.ext_urls.is_empty()
    }
}

impl fmt::Debug for Sauce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result: String = String::new();
        result.push_str("ext_urls: ");
        for i in self.ext_urls.clone() {
            result.push_str(format!("\"{}\"", i.as_str()).as_str());
            result.push_str("  ");
        }
        result.push_str("\nsite: ");
        result.push_str(self.site.as_str());
        match &self.title {
            Some(x) => {
                result.push_str("\ntitle: ");
                result.push_str(x.as_str());
            }
            None => (),
        }
        result.push_str("\nindex: ");
        result.push_str(self.index.to_string().as_str());
        result.push_str("\nindex_id: ");
        result.push_str(self.index_id.to_string().as_str());
        result.push_str("\nsimilarity: ");
        result.push_str(self.similarity.to_string().as_str());
        result.push_str("\nthumbnail: ");
        result.push_str(format!("\"{}\"", self.thumbnail.as_str()).as_str());
        result.push_str("\nauthor_id: ");
        if let Some(author) = self.additional_fields.clone() {
            result.push_str(author.to_string().as_str());
        }

        write!(f, "{}", result)
    }
}

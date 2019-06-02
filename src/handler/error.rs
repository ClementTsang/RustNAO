use std::error::Error;
use std::fmt;
extern crate serde_json;
extern crate reqwest;

#[derive(Debug)]
pub struct SauceError {
	details: String,
}

impl SauceError {
	pub fn new(msg : &str) -> SauceError { 
		SauceError {
			details: msg.to_string(),
		}
	}
}

impl fmt::Display for SauceError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
		write!(f, "{}", self.details)
	}
}

impl Error for SauceError {
	fn description(&self) -> &str {
		&self.details
	}
}

//TODO: More robust error descriptions...

impl From<serde_json::Error> for SauceError {
	fn from(err : serde_json::Error) -> Self {
		SauceError::new(format!("serde_json Error: {}", err.description()).as_str())
	}
}

impl From<reqwest::Error> for SauceError {
	fn from(err : reqwest::Error) -> Self {
		SauceError::new(format!("reqwest Error: {}", err.description()).as_str())
	}
}

impl From<reqwest::UrlError> for SauceError {
	fn from(err : reqwest::UrlError) -> Self {
		SauceError::new(format!("reqwest UrlError: {}", err.description()).as_str())
	}
}
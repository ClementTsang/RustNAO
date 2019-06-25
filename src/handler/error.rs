//! Error components for the library.
// Much thanks to Andrew Gallant for the basis of this part of the library... followed the following
// code for this: https://github.com/BurntSushi/imdb-rename/blob/master/imdb-index/src/error.rs.

use std::fmt;
use std::result;
use failure::{Backtrace, Context, Fail};

/// A type alias for handling errors related to rustnao.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur while interacting to the SauceNAO API.
#[derive(Debug)]
pub struct Error {
	context : Context<ErrType>,
}

impl Error {
	/// Return the kind of error
	pub fn kind(&self) -> &ErrType {
		self.context.get_context()
	}

	pub(crate) fn invalid_url<T: AsRef<str>>(unk: T) -> Error {
		Error::from(ErrType::InvalidURL(unk.as_ref().to_string()))
	}

	pub(crate) fn invalid_path<T: AsRef<str>>(unk: T) -> Error {
		Error::from(ErrType::InvalidFile(unk.as_ref().to_string()))
	}

	pub(crate) fn invalid_serde<T: AsRef<str>>(unk: T) -> Error {
		Error::from(ErrType::InvalidSerde(unk.as_ref().to_string()))
	}

	pub(crate) fn invalid_code(code : i32, message : String) -> Error {
		Error::from(ErrType::InvalidCode{code, message})
	}

	pub(crate) fn invalid_request<T: AsRef<str>>(unk: T) -> Error {
		Error::from(ErrType::InvalidRequest(unk.as_ref().to_string()))
	}

	pub(crate) fn invalid_parameter(message : String) -> Error {
		Error::from(ErrType::InvalidParameters(message))
	}
}

impl Fail for Error {
	fn cause(&self) -> Option<&Fail> {
		self.context.cause()
	}

	fn backtrace(&self) -> Option<&Backtrace> {
		self.context.backtrace()
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		self.context.fmt(f)
	}
}

/// The specific type of error that can occur.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrType {
	/// An error when forming the URL for the API.  
	/// 
	/// The data provided is the error found.
	InvalidURL(String),
	/// An error when getting the file path of a file for the API.  
	/// 
	/// The data provided is the error found.
	InvalidFile(String),
	/// An error when trying to deserialize the resulting JSON from the API
	/// 
	/// The data provided is the error found.
	InvalidSerde(String),
	/// An error when receiving an unsuccessful code from the SauceNAO API.
	/// 
	/// The data provided is the error code and message.
	InvalidCode {
		/// The error code from SauceNAO
		code : i32, 
		/// The message showing the cause of the error from SauceNAO
		message : String,
	},
	/// An error when trying to send an invalid request to the API.
	/// 
	/// The data provided is the error code and message.
	InvalidRequest(String),
	/// An error with some data that is passed in by the user.
	/// 
	/// The data provided is an error message.
	InvalidParameters(String),
}

impl fmt::Display for ErrType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ErrType::InvalidURL(ref unk) => write!(f, "ERROR: URL was invalid, error was due to: {}", unk),
			ErrType::InvalidFile(ref unk) => write!(f, "ERROR: File path was invalid, error was due to: {}", unk),
			ErrType::InvalidSerde(ref unk) => write!(f, "ERROR: Could not properly serde results: {}", unk),
			ErrType::InvalidCode {code, message} => write!(f, "ERROR: Recieved an invalid status code {} after API call with message: \"{}\"", code, message),
			ErrType::InvalidRequest(ref unk) => write!(f, "ERROR: Failed to make the request, error was due to: {}", unk),
			ErrType::InvalidParameters(message) => write!(f, "ERROR: An invalid parameter was passed, error was due to: {}", message),
		}
	}
}

impl From<ErrType> for Error {
    fn from(err_type: ErrType) -> Error {
        Error::from(Context::new(err_type))
    }
}

impl From<Context<ErrType>> for Error {
    fn from(context: Context<ErrType>) -> Error {
        Error { context }
    }
}

impl From<serde_json::Error> for Error {
	fn from(err : serde_json::Error) -> Self {
		Error::invalid_serde(err.to_string())
	}
}

impl From<reqwest::Error> for Error {
	fn from(err : reqwest::Error) -> Self {
		Error::invalid_request(err.to_string())
	}
}

impl From<reqwest::UrlError> for Error {
	fn from(err : reqwest::UrlError) -> Self {
		Error::invalid_url(err.to_string())
	}
}

impl From<std::io::Error> for Error {
	fn from(err : std::io::Error) -> Self {
		Error::invalid_path(err.to_string())
	}
}
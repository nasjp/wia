use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
	IoError(io::Error),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use Error::*;
		match self {
			IoError(err) => write!(f, "{}", err),
		}
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::IoError(err)
	}
}

pub type Result<T> = std::result::Result<T, Error>;
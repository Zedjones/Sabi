extern crate reqwest;

use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, SabiError>;

#[derive(Debug)]
pub enum SabiError {
    InvalidWord(String),
    NetworkError(reqwest::Error)
}

impl error::Error for SabiError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            SabiError::NetworkError(error) => Some(error),
            SabiError::InvalidWord(_) => None
        }
    }
}

impl fmt::Display for SabiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SabiError::NetworkError(error) => error.fmt(f),
            SabiError::InvalidWord(word) => {
                write!(f, "No results found for {}.", word)
            }
        }
    }
}
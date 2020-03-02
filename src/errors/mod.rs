extern crate reqwest;

use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, SabiError>;

#[derive(Debug)]
pub struct SabiError {
    error: reqwest::Error
}

impl SabiError {
    pub fn new(error: reqwest::Error) -> SabiError {
        SabiError {error}
    }
}

impl error::Error for SabiError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.error)
    }
}

impl fmt::Display for SabiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.error.fmt(f)
    }
}
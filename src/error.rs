use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub struct Error;

impl error::Error for Error {
    fn description(&self) -> &str {
        "error"
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = result::Result<T, Error>;

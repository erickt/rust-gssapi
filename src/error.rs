use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub struct Error {
    major_status: u32,
    minor_status: u32,
}

impl Error {
    pub fn new(major_status: u32, minor_status: u32) -> Self {
        Error {
            major_status: major_status,
            minor_status: minor_status,
        }
    }
}

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

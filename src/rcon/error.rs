
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RCONError {
    pub message: String,
}

impl Error for RCONError {}

impl fmt::Display for RCONError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
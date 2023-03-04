use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct APIError {
    pub message: String,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "APIError: {}", self.message)
    }
}

impl Error for APIError {}

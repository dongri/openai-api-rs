use reqwest::{self};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum APIError {
    ReqwestError(reqwest::Error),
    CustomError { message: String },
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIError::ReqwestError(err) => write!(f, "ReqwestError: {}", err),
            APIError::CustomError { message } => write!(f, "APIError: {}", message),
        }
    }
}

impl Error for APIError {}

impl From<reqwest::Error> for APIError {
    fn from(err: reqwest::Error) -> APIError {
        APIError::ReqwestError(err)
    }
}

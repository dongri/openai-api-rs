#[cfg(not(all(target_arch = "wasm32", target_os = "wasi")))]
use reqwest::{self};

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum APIError {
    #[cfg(not(all(target_arch = "wasm32", target_os = "wasi")))]
    ReqwestError(reqwest::Error),
    #[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
    ReqwestError(anyhow::Error),
    CustomError { message: String },
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIError::ReqwestError(err) => write!(f, "ReqwestError: {err}"),
            APIError::CustomError { message } => write!(f, "APIError: {message}"),
        }
    }
}

impl Error for APIError {}

#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
impl From<anyhow::Error> for APIError {
    fn from(err: anyhow::Error) -> APIError {
        APIError::ReqwestError(err)
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "wasi")))]
impl From<reqwest::Error> for APIError {
    fn from(err: reqwest::Error) -> APIError {
        APIError::ReqwestError(err.into())
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
impl From<golem_wasi_http::Error> for APIError {
    fn from(err: golem_wasi_http::Error) -> APIError {
        APIError::ReqwestError(err.into())
    }
}

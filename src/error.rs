#![allow(clippy::write_with_newline)]

//! Custom errors from this crate.
use std::{error, fmt, io, path::PathBuf};

// Imports with bindings improve how Error is shown in docs
use io::Error as IoError;
use reqwest::{Error as ReqwestError, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use toml::de::Error as SerdeTomlError;

use crate::utils;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Reqwest { url: String, reason: ReqwestError },
    Json(SerdeJsonError),
    Io(IoError),
    Toml { path: PathBuf, reason: SerdeTomlError },
    Response { request_url: String, reason: ResponseError },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Reqwest { url, reason } => {
                write!(f, "reqwest error:\n")?;
                write!(f, "  url: {}\n", url)?;
                write!(f, "  reason: {}", reason)
            }
            Error::Json(source) => write!(f, "json err: {}.", source),
            Error::Io(source) => write!(f, "io err: {}.", source),
            Error::Toml { path, reason } => {
                write!(f, "toml err:\n")?;
                write!(f, "url: {}\n", path.display())?;
                write!(f, "reason: {}.", reason)
            }
            Error::Response { reason, request_url } => {
                write!(f, "response error:\n")?;
                write!(f, "  url: {}\n", request_url)?;
                reason.fmt(f)
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Reqwest { reason, .. } => Some(reason),
            Error::Json(source) => Some(source),
            Error::Io(source) => Some(source),
            Error::Toml { reason, .. } => Some(reason),
            Error::Response { reason, .. } => Some(reason),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseError {
    status_code: u16,
    error: String,
    message: String,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  status code: {}\n", self.status_code)?;
        write!(f, "  error: {}\n", self.error)?;
        write!(f, "  message: {}", self.message)
    }
}

impl error::Error for ResponseError {}

impl From<SerdeJsonError> for Error {
    fn from(source: SerdeJsonError) -> Self {
        Error::Json(source)
    }
}

impl From<IoError> for Error {
    fn from(source: IoError) -> Self {
        Error::Io(source)
    }
}

// Parsing the error response is tricky, it's necessary to check if the json body is
// malformed, if so, we will catch an error trying to get the cause to another error
//
// Catching a Error::Json when trying to interpret a Error::ErrorResponse
//
// This function can only return Error::ErrorResponse.
pub(crate) fn process_error_response(text: &str, status_code: StatusCode, url: &str) -> Error {
    let status_code = status_code.as_u16();

    let expected_error_codes = &[400, 403, 404, 418, 429, 500];
    if !expected_error_codes.contains(&status_code) {
        eprintln!("Warning: status code {} was not expected.", status_code);
    }
    let request_url = url.into();

    match serde_json::from_str::<ResponseError>(text) {
        Ok(http_error) => Error::Response { reason: http_error, request_url },
        Err(_) => {
            // Try to format JSON body, or use unformatted body instead
            let formatted_body_text =
                utils::try_formatting_json(text).unwrap_or_else(|_| text.to_owned());
            let reason = "Could not parse error body to interpret the reason of the error".into();

            let http_error =
                ResponseError { status_code, error: reason, message: formatted_body_text };
            Error::Response { reason: http_error, request_url }
        }
    }
}

// Helper to create a Error::Reqwest
pub(crate) fn reqwest_error(url: impl ToString, error: reqwest::Error) -> Error {
    Error::Reqwest { url: url.to_string(), reason: error }
}

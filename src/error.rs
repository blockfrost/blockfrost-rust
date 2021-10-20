use std::{error, fmt, io};

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

// Imports with bindings improve how Error is shown in docs
use io::Error as IoError;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use toml::de::Error as SerdeTomlError;

use crate::utils;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Reqwest(ReqwestError),
    Json(SerdeJsonError),
    Io(IoError),
    Toml(SerdeTomlError),
    Response(ResponseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Reqwest(source) => write!(f, "http err: {}.", source),
            Error::Json(source) => write!(f, "json err: {}.", source),
            Error::Io(source) => write!(f, "io err: {}.", source),
            Error::Toml(source) => write!(f, "toml err: {}.", source),
            Error::Response(source) => source.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Reqwest(source) => Some(source),
            Error::Json(source) => Some(source),
            Error::Io(source) => Some(source),
            Error::Toml(source) => Some(source),
            Error::Response(source) => Some(source),
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
        writeln!(f, "http error:")?;
        writeln!(f, "  status code: {}", self.status_code)?;
        writeln!(f, "  error: {}", self.error)?;
        write!(f, "  message: {}", self.message)
    }
}

impl error::Error for ResponseError {}

impl From<ReqwestError> for Error {
    fn from(source: ReqwestError) -> Self {
        Error::Reqwest(source)
    }
}

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

impl From<SerdeTomlError> for Error {
    fn from(source: SerdeTomlError) -> Self {
        Error::Toml(source)
    }
}

impl From<ResponseError> for Error {
    fn from(source: ResponseError) -> Self {
        Error::Response(source)
    }
}

// Parsing the error response is tricky, it's necessary to check if the json body is
// malformed, if so, we will catch an error trying to get the cause to another error
//
// Catching a Error::Json when trying to interpret a Error::ErrorResponse
//
// TODO: CHANGE ERROR_RESPONSE NAME
//
// This function can only return Error::ErrorResponse.
pub(crate) fn process_error_response(text: &str, status_code: StatusCode) -> Error {
    let status_code = status_code.as_u16();

    let expected_error_codes = &[400, 403, 404, 418, 429, 500];
    if !expected_error_codes.contains(&status_code) {
        eprintln!("Warning: status code {} was not expected.", status_code);
    }

    match serde_json::from_str::<ResponseError>(text) {
        Ok(http_error) => Error::Response(http_error),
        Err(_) => {
            // Try to format JSON body, or use unformatted body instead
            let formatted_body_text =
                utils::try_formatting_json(text).unwrap_or_else(|_| text.to_owned());
            let reason = "Could not parse error body to interpret the reason of the error".into();

            let http_error =
                ResponseError { status_code, error: reason, message: formatted_body_text };
            Error::Response(http_error)
        }
    }
}

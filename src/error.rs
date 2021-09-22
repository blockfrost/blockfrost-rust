use std::path::PathBuf;
use std::{error, fmt, io};

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

// Imports with bindings improve how Error is shown in docs
use io::Error as IoError;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;

use crate::utils;

#[derive(Debug)]
pub enum Error {
    Network(ReqwestError),
    Json(SerdeJsonError),
    Io(IoError),
    DotEnv(DotEnvError),
    Http(HttpError),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpError {
    status_code: u16,
    error: String,
    message: String,
}

#[derive(Debug, Clone)]
pub struct DotEnvError {
    pub reason: &'static str,
    pub path: PathBuf,
    pub line_number: usize,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Network(source) => write!(f, "http err: {}.", source),
            Error::Json(source) => write!(f, "json err: {}.", source),
            Error::Io(source) => write!(f, "io err: {}.", source),
            Error::DotEnv(source) => source.fmt(f),
            Error::Http(source) => source.fmt(f),
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "http error:")?;
        writeln!(f, "  status code: {}", self.status_code)?;
        writeln!(f, "  error: {}", self.error)?;
        write!(f, "  message: {}", self.message)
    }
}

impl fmt::Display for DotEnvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "dotenv err: parse error at line {} of file '{:#?}': {}.", self.line_number, self.path, self.reason)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Network(source) => source,
            Error::Json(source) => source,
            Error::Io(source) => source,
            Error::DotEnv(source) => source,
            Error::Http(source) => source,
        })
    }
}

impl error::Error for HttpError {}

impl error::Error for DotEnvError {}

pub type Result<T> = std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Error::Network(source)
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error::Json(source)
    }
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        Error::Io(source)
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

    match serde_json::from_str::<HttpError>(text) {
        Ok(http_error) => Error::Http(http_error),
        Err(_) => {
            // Try to format JSON body, or use unformatted body instead
            let formatted_body_text = utils::try_formatting_json(text).unwrap_or_else(|_| text.to_owned());
            let reason = "Could not parse error body to interpret the reason of the error".into();

            let http_error = HttpError { status_code, error: reason, message: formatted_body_text };
            Error::Http(http_error)
        }
    }
}

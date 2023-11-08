use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::{error, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockfrostError {
    #[error("Reqwest error for URL {url}: {reason}")]
    Reqwest { url: String, reason: reqwest::Error },
    #[error("JSON error for URL {url}: {reason}\nText: '{text}'")]
    Json {
        url: String,
        text: String,
        reason: serde_json::Error,
    },
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Response error for URL {url}: {reason}")]
    Response { url: String, reason: ResponseError },
}

#[derive(Serialize, Deserialize, Error, Debug, Clone)]
pub struct ResponseError {
    pub status_code: u16,
    pub error: String,
    pub message: String,
}
impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Status code: {}", self.status_code)?;
        writeln!(f, "Error: {}", self.error)?;
        write!(f, "Message: {}", self.message)
    }
}

impl error::Error for ResponseError {}

impl From<IoError> for BlockfrostError {
    fn from(source: IoError) -> Self {
        BlockfrostError::Io(source)
    }
}

// Parsing the error response is tricky, it's necessary to check if the json body is
// malformed, if so, we will catch an error trying to get the cause to another error
//
// Catching a Error::Json when trying to interpret a Error::ErrorResponse
//
// This function can only return Error::ErrorResponse.
pub(crate) fn process_error_response(
    text: &str,
    status_code: StatusCode,
    url: &str,
) -> BlockfrostError {
    let status_code = status_code.as_u16();

    let expected_error_codes = &[400, 403, 404, 418, 429, 500];
    if !expected_error_codes.contains(&status_code) {
        eprintln!("Warning: status code {} was not expected.", status_code);
    }
    let url = url.into();

    match json_from::<ResponseError>(text) {
        Ok(http_error) => BlockfrostError::Response {
            reason: http_error,
            url,
        },
        Err(_) => {
            // Try to format JSON body, or use unformatted body instead
            let formatted_body_text =
                utils::try_formatting_json(text).unwrap_or_else(|_| text.to_owned());
            let reason = "Could not parse error body to interpret the reason of the error".into();

            let http_error = ResponseError {
                status_code,
                error: reason,
                message: formatted_body_text,
            };
            BlockfrostError::Response {
                reason: http_error,
                url,
            }
        },
    }
}

// Helper to create a Error::Reqwest
pub(crate) fn reqwest_error(url: impl ToString, error: ReqwestError) -> BlockfrostError {
    BlockfrostError::Reqwest {
        url: url.to_string(),
        reason: error,
    }
}

// Helper to create a Error::Json
pub(crate) fn json_error(
    url: impl ToString,
    text: impl ToString,
    error: SerdeJsonError,
) -> BlockfrostError {
    BlockfrostError::Json {
        url: url.to_string(),
        text: text.to_string(),
        reason: error,
    }
}

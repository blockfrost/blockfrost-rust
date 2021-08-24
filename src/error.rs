use std::path::PathBuf;
use std::{error, fmt, io};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    Json(serde_json::Error),
    Io(io::Error),
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
        write!(
            f,
            "dotenv err: parse error at line {} of file '{:#?}': {}.",
            self.line_number, self.path, self.reason
        )
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

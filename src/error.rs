use std::path::PathBuf;
use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Io(io::Error),
    DotEnv {
        reason: &'static str,
        path: PathBuf,
        line_number: usize,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Http(source) => write!(f, "http err: {:#?}.", source),
            Error::Json(source) => write!(f, "json err: {:#?}.", source),
            Error::Io(source) => write!(f, "io err: {:#?}.", source),
            Error::DotEnv {
                reason,
                path,
                line_number,
            } => write!(
                f,
                "dotenv err: parse error at line {} of file '{:#?}': {}.",
                line_number, path, reason
            ),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Http(source) => Some(source),
            Error::Json(source) => Some(source),
            Error::Io(source) => Some(source),
            Error::DotEnv { .. } => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Error::Http(source)
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

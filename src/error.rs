use std::fmt;

/// Errors returned by the SDK.
#[derive(Debug)]
pub enum Error {
    /// The API returned a non-success status code.
    Api {
        message: String,
        code: u16,
        response: String,
        kind: String,
    },
    /// The underlying HTTP transport failed.
    Http(reqwest::Error),
    /// (De)serialization of a request or response payload failed.
    Serde(serde_json::Error),
    /// A local I/O operation (e.g. reading a file to upload) failed.
    Io(std::io::Error),
}

impl Error {
    /// The HTTP status code, when the error originated from an API response.
    pub fn code(&self) -> Option<u16> {
        match self {
            Error::Api { code, .. } => Some(*code),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Api { message, code, .. } => write!(f, "API error ({}): {}", code, message),
            Error::Http(e) => write!(f, "HTTP error: {}", e),
            Error::Serde(e) => write!(f, "serialization error: {}", e),
            Error::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Http(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

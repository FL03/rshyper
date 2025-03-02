/*
    Appellation: error <module>
    Contrib: @FL03
*/

/// A type alias for a [Result] with the crate-specific error type [Error]
pub type Result<T = ()> = core::result::Result<T, Error>;

/// The error type for this crate
#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    IOError(String),
    #[error("Parse Error: {0}")]
    ParseError(String),
    #[error("Unknown Error: {0}")]
    Unknown(String),
}


impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Unknown(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        Error::Unknown(e.to_string())
    }
}
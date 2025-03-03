/*
    Appellation: error <module>
    Contrib: @FL03
*/

/// A type alias for a [Result] with the crate-specific error type [Error]
pub type Result<T = ()> = core::result::Result<T, Error>;

/// The error type for this crate
#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("Graph error: {0}")]
    GraphError(#[from] rshyper::Error),
    #[error("Turing error: {0}")]
    TuringError(#[from] rstm::Error),
    #[error("Invalid State: {0}")]
    InvalidState(String),
    #[error("Invalid Symbol: {0}")]
    InvalidSymbol(String),
    #[error("Infinite loop detected")]
    InfiniteLoop,
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

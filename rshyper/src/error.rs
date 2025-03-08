/*
    Appellation: error <module>
    Contrib: @FL03
*/
use crate::{EdgeId, VertexId};

/// A type alias for a [Result] with the crate-specific error type [Error]
pub type Result<T = ()> = core::result::Result<T, Error>;

/// The error type for this crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot create empty hyperedge")]
    EmptyHyperedge,
    #[error("Hyperedge {0} does not exist")]
    HyperedgeDoesNotExist(EdgeId),
    #[error("Vertex {0} does not exist")]
    VertexDoesNotExist(VertexId),
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Unknown(s.to_string())
    }
}
#[cfg(feature = "alloc")]
impl From<alloc::string::String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}

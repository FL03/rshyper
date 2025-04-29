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
    #[cfg(feature = "anyhow")]
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    Boxed(#[from] alloc::boxed::Box<dyn core::error::Error + 'static + Send + Sync>),
    #[cfg(feature = "alloc")]
    #[error("Unknown error: {0}")]
    Unknown(alloc::string::String),
}

#[cfg(feature = "alloc")]
impl From<alloc::string::String> for Error {
    fn from(value: alloc::string::String) -> Self {
        Self::Unknown(value)
    }
}

#[cfg(feature = "alloc")]
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Unknown(value.to_string())
    }
}
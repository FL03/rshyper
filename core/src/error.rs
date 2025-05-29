/*
    Appellation: error <module>
    Contrib: @FL03
*/
//! this module implements the [`Error`] type for the [`rshyper`](https://docs.rs/rshyper)
//! crate.

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, string::String};

use crate::{EdgeId, VertexId};

/// A type alias for a [Result] with the crate-specific error type [Error]
pub type Result<T = ()> = core::result::Result<T, Error>;

/// The error type for this crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot create empty hyperedge")]
    EmptyHyperedge,
    #[error("Hyperedge {0} does not exist")]
    HyperedgeDoesNotExist(EdgeId<usize>),
    #[error("Vertex {0} does not exist")]
    VertexDoesNotExist(VertexId<usize>),
    #[cfg(feature = "anyhow")]
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[cfg(feature = "serde_json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    Other(#[from] Box<dyn core::error::Error + Send + Sync + 'static>),
    #[cfg(feature = "alloc")]
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[cfg(feature = "alloc")]
impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Unknown(String::from(s))
    }
}

#[cfg(feature = "alloc")]
impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}

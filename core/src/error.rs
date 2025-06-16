/*
    Appellation: error <module>
    Contrib: @FL03
*/
//! this module implements the [`Error`] type for the [`rshyper`](https://docs.rs/rshyper)
//! crate.
#[cfg(feature = "alloc")]
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
/// A type alias for a [Result] with the crate-specific error type [`Error`]
pub type Result<T = ()> = core::result::Result<T, Error>;

/// The error type for this crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "alloc")]
    #[error("The edge with the given id does not exist")]
    EdgeNotFound,
    #[error("The node with the given id does not exist")]
    NodeNotFound,
    #[error("No edges contain the given vertex")]
    NoEdgesWithVertex,
    #[error("Cannot create an empty hyperedge")]
    EmptyHyperedge,
    #[error(transparent)]
    IndexError(#[from] crate::idx::IndexError),
    #[cfg(feature = "anyhow")]
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] Box<dyn core::error::Error + Send + Sync>),
    #[cfg(feature = "serde")]
    #[error(transparent)]
    DeserializeError(#[from] serde::de::value::Error),
    #[error(transparent)]
    FmtError(#[from] core::fmt::Error),
    #[cfg(feature = "std")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[cfg(feature = "serde_json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[cfg(feature = "alloc")]
    #[error("{0}")]
    Unknown(String),
}

impl Error {
    /// returns a new [`EdgeNotFound`](Error::EdgeNotFound) variant
    pub fn edge_not_found() -> Self {
        Error::EdgeNotFound
    }
    /// returns a new [`NodeNotFound`](Error::NodeNotFound) variant
    pub fn node_not_found() -> Self {
        Error::NodeNotFound
    }
    #[cfg(feature = "alloc")]
    /// returns a new [`Unknown`](Error::Unknown) variant initialized with the given value
    pub fn unknown<S: ToString>(s: S) -> Self {
        Error::Unknown(s.to_string())
    }
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

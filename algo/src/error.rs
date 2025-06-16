/*
    Appellation: error <module>
    Contrib: @FL03
*/
//! this module implements the [`Error`] type for algorithms and operators for hypergraphs in
//! the [`rshyper`](https://docs.rs/rshyper) crate.

#[cfg(feature = "alloc")]
use alloc::string::ToString;
use rshyper_core::error::Error as CoreError;
/// a type alias for a [Result] with the crate-specific error type [`AlgoError`]
pub type Result<T = ()> = core::result::Result<T, Error>;

/// The [`Error`] type enumerates the various errors encountered by algorithms and operators on
/// hypergraphs
#[derive(Debug, strum::EnumIs, thiserror::Error)]
pub enum Error {
    #[error("No path found between the two points")]
    PathNotFound,
    #[error("The edge with the given id does not exist")]
    EdgeNotFound,
    #[error("The node with the given id does not exist")]
    NodeNotFound,
    #[error("No edges contain the given vertex")]
    NoEdgesWithVertex,
    #[error("Cannot create an empty hyperedge")]
    EmptyHyperedge,
    #[error(transparent)]
    CoreError(#[from] CoreError),
}

impl Error {
    fn variant(&self) -> &str {
        match self {
            Error::PathNotFound => "No path found between the two points",
            Error::EdgeNotFound => "The edge with the given id does not exist",
            Error::NodeNotFound => "The node with the given id does not exist",
            Error::NoEdgesWithVertex => "No edges contain the given vertex",
            Error::EmptyHyperedge => "Cannot create an empty hyperedge",
            Error::CoreError(_e) => "core error",
        }
    }
}
#[cfg(feature = "alloc")]
impl From<Error> for CoreError {
    fn from(e: Error) -> Self {
        match e {
            Error::CoreError(e) => e,
            _ => CoreError::Unknown(e.variant().to_string()),
        }
    }
}

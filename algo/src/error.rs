/*
    Appellation: error <module>
    Contrib: @FL03
*/
//! this module implements the [`Error`] type for the [`rshyper`](https://docs.rs/rshyper)
//! crate.

/// A type alias for a [Result] with the crate-specific error type [Error]
pub(crate) type Result<T = ()> = core::result::Result<T, Error>;

/// The error type for this crate
#[derive(Debug, thiserror::Error)]
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
    CoreError(#[from] rshyper_core::error::Error),
}

impl From<Error> for rshyper_core::error::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::CoreError(e) => e,
            Error::PathNotFound => rshyper_core::error::Error::PathNotFound,
            Error::EdgeNotFound => rshyper_core::error::Error::EdgeNotFound,
            Error::NodeNotFound => rshyper_core::error::Error::NodeNotFound,
            Error::NoEdgesWithVertex => rshyper_core::error::Error::NoEdgesWithVertex,
            Error::EmptyHyperedge => rshyper_core::error::Error::EmptyHyperedge,
        }
    }
}

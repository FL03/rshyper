/*
    Appellation: error <module>
    Contrib: @FL03
*/
//! this module implements the [`AlgoError`] type for algorithms in the [`rshyper`](https://docs.rs/rshyper)
//! crate.

/// A type alias for a [Result] with the crate-specific error type [`AlgoError`]
pub type AlgoResult<T = ()> = core::result::Result<T, AlgoError>;

/// The error type for this crate
#[derive(Debug, thiserror::Error)]
pub enum AlgoError {
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
    CoreError(#[from] rshyper_core::error::HyperError),
}

impl From<AlgoError> for rshyper_core::error::HyperError {
    fn from(e: AlgoError) -> Self {
        match e {
            AlgoError::CoreError(e) => e,
            AlgoError::PathNotFound => rshyper_core::error::HyperError::PathNotFound,
            AlgoError::EdgeNotFound => rshyper_core::error::HyperError::EdgeNotFound,
            AlgoError::NodeNotFound => rshyper_core::error::HyperError::NodeNotFound,
            AlgoError::NoEdgesWithVertex => rshyper_core::error::HyperError::NoEdgesWithVertex,
            AlgoError::EmptyHyperedge => rshyper_core::error::HyperError::EmptyHyperedge,
        }
    }
}

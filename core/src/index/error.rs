/*
    Appellation: error <module>
    Contrib: @FL03
*/
//! this module implements the [`Error`] type for the [`rshyper`](https://docs.rs/rshyper)
//! crate.

use crate::{EdgeId, VertexId};

/// A type alias for a [Result] with the crate-specific error type [`IndexError`]
pub type IndexResult<T = ()> = core::result::Result<T, IndexError>;

/// The error type for this crate
#[derive(Debug, thiserror::Error)]
pub enum IndexError {
    #[error("The index does not exist")]
    IndexNotFound,
    #[error("Index is out of bounds")]
    IndexOutOfBounds,
    #[error("Invalid index")]
    InvalidIndex,
    #[error("No path found between {from} and {to}")]
    NoPathFoundBetween {
        #[source]
        source: Option<Box<dyn core::error::Error + Send + Sync>>,
        from: VertexId<usize>,
        to: VertexId<usize>,
    },
    #[error("Hyperedge {0} does not exist")]
    HyperedgeDoesNotExist(EdgeId<usize>),
    #[error("Vertex {0} does not exist")]
    VertexDoesNotExist(VertexId<usize>),
}

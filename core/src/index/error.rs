/*
    Appellation: error <module>
    Contrib: @FL03
*/
use super::{EdgeId, RawIndex, VertexId};
#[cfg(feature = "alloc")]
use alloc::string::String;

/// A type alias for a [Result] with the crate-specific error type [`IndexError`]
pub type IndexResult<T = ()> = core::result::Result<T, IndexError>;

/// The error type for index-related operations
#[derive(Debug, thiserror::Error)]
pub enum IndexError {
    #[error("Index is out of bounds")]
    IndexOutOfBounds,
    #[error("Invalid index")]
    InvalidIndex,
    #[cfg(feature = "alloc")]
    #[error("The index does not exist")]
    IndexNotFound(String),
    #[cfg(feature = "alloc")]
    #[error("No path found between {from} and {to}")]
    NoPathFoundBetween { from: String, to: String },
    #[cfg(feature = "alloc")]
    #[error("Hyperedge {0} does not exist")]
    HyperedgeDoesNotExist(String),
    #[cfg(feature = "alloc")]
    #[error("Vertex {0} does not exist")]
    VertexDoesNotExist(String),
}

impl IndexError {
    /// initialize a new [`IndexOutOfBounds`](IndexError::IndexOutOfBounds) error variant
    pub fn index_out_of_bounds() -> Self {
        IndexError::IndexOutOfBounds
    }
    /// initialize a new [`InvalidIndex`](IndexError::InvalidIndex) error variant
    pub fn invalid_index() -> Self {
        IndexError::InvalidIndex
    }
    /// initialize a new [`IndexNotFound`](IndexError::IndexNotFound) error variant using the
    /// raw inner value of some index.
    #[cfg(feature = "alloc")]
    pub fn index_not_found<Idx: RawIndex>(index: Idx) -> Self {
        IndexError::IndexNotFound(index.to_string())
    }
    /// initialize a new [`NoPathFoundBetween`](IndexError::NoPathFoundBetween) error variant
    #[cfg(feature = "alloc")]
    pub fn no_path_found_between<Idx: RawIndex>(from: Idx, to: Idx) -> Self {
        IndexError::NoPathFoundBetween {
            from: from.to_string(),
            to: to.to_string(),
        }
    }
    /// initialize a new [`HyperedgeDoesNotExist`](IndexError::HyperedgeDoesNotExist) error
    /// variant
    #[cfg(feature = "alloc")]
    pub fn hyperedge_does_not_exist<Idx: RawIndex>(index: EdgeId<Idx>) -> Self {
        IndexError::HyperedgeDoesNotExist(index.get().to_string())
    }
    /// initialize a new [`VertexDoesNotExist`](IndexError::VertexDoesNotExist) error variant
    #[cfg(feature = "alloc")]
    pub fn vertex_does_not_exist<Idx: RawIndex>(index: VertexId<Idx>) -> Self {
        IndexError::VertexDoesNotExist(index.get().to_string())
    }
}

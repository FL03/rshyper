/*
    Appellation: error <module>
    Contrib: @FL03
*/
use super::{EdgeId, RawIndex, VertexId};

use alloc::boxed::Box;

/// A type alias for a [`Result`] with an error type of [`IndexError`]
pub type IndexResult<T = ()> = core::result::Result<T, IndexError>;

/// The error type for index-related operations
#[derive(Debug, thiserror::Error)]
pub enum IndexError {
    #[error("Duplicate index")]
    DuplicateIndex,
    #[error("Index is out of bounds")]
    IndexOutOfBounds,
    #[error("The index ({0}) is invalid")]
    InvalidIndex(Box<dyn RawIndex>),
    #[error("The index {0} does not exist")]
    IndexNotFound(Box<dyn RawIndex>),
    #[error("No path found between {from} and {to}")]
    NoPathFoundBetween {
        from: Box<dyn RawIndex>,
        to: Box<dyn RawIndex>,
    },
    #[error("Hyperedge {0} does not exist")]
    HyperedgeDoesNotExist(Box<dyn RawIndex>),
    #[error("Vertex {0} does not exist")]
    VertexDoesNotExist(Box<dyn RawIndex>),
}

impl IndexError {
    /// initialize a new [`IndexOutOfBounds`](IndexError::IndexOutOfBounds) error variant
    pub fn index_out_of_bounds() -> Self {
        IndexError::IndexOutOfBounds
    }
    /// initialize a new [`InvalidIndex`](IndexError::InvalidIndex) error variant
    pub fn invalid_index<Ix: RawIndex>(value: Ix) -> Self {
        IndexError::InvalidIndex(Box::new(value))
    }
    /// initialize a new [`IndexNotFound`](IndexError::IndexNotFound) error variant using the
    /// raw inner value of some index.
    pub fn index_not_found<Idx: RawIndex>(value: Idx) -> Self {
        IndexError::IndexNotFound(Box::new(value))
    }
    /// initialize a new [`NoPathFoundBetween`](IndexError::NoPathFoundBetween) error variant
    pub fn no_path_found_between<Idx: RawIndex>(from: Idx, to: Idx) -> Self {
        IndexError::NoPathFoundBetween {
            from: Box::new(from),
            to: Box::new(to),
        }
    }
    /// initialize a new [`HyperedgeDoesNotExist`](IndexError::HyperedgeDoesNotExist) error
    /// variant
    pub fn hyperedge_does_not_exist<Idx: RawIndex>(index: EdgeId<Idx>) -> Self {
        IndexError::HyperedgeDoesNotExist(Box::new(index.value()))
    }
    /// initialize a new [`VertexDoesNotExist`](IndexError::VertexDoesNotExist) error variant
    pub fn vertex_does_not_exist<Idx: RawIndex>(index: VertexId<Idx>) -> Self {
        IndexError::VertexDoesNotExist(index.into_raw_box())
    }
}

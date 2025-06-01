/*
    Appellation: error <module>
    Contrib: @FL03
*/
use super::{EdgeId, RawIndex, VertexId};

/// A type alias for a [Result] with the crate-specific error type [`IndexError`]
pub type IndexResult<T = (), Idx = usize> = core::result::Result<T, IndexError<Idx>>;

/// The error type for this crate
#[derive(Debug, thiserror::Error)]
pub enum IndexError<Idx = usize>
where
    Idx: RawIndex,
{
    #[error("The index does not exist")]
    IndexNotFound(Idx),
    #[error("Index is out of bounds")]
    IndexOutOfBounds(Idx),
    #[error("Invalid index")]
    InvalidIndex,
    #[error("No path found between {from} and {to}")]
    NoPathFoundBetween { from: Idx, to: Idx },
    #[error("Hyperedge {0} does not exist")]
    HyperedgeDoesNotExist(EdgeId<Idx>),
    #[error("Vertex {0} does not exist")]
    VertexDoesNotExist(VertexId<Idx>),
}

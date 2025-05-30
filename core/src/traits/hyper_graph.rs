/*
    Appellation: hgraph <module>
    Contrib: @FL03
*/
use crate::{EdgeId, RawIndex, VertexId};

pub trait RawNode {
    type Idx: RawIndex;

    private!();

    /// Returns the index of the node.
    fn index(&self) -> &VertexId<Self::Idx>;
}

pub trait RawEdge {
    type Idx: RawIndex;

    private!();

    /// Returns the index of the node.
    fn index(&self) -> &EdgeId<Self::Idx>;
}

pub trait RawHyperGraph<N, E> {
    type Idx: RawIndex;
}

/// [`HyperGraph`] is a trait that defines the basic operations for a hypergraph data structure.
pub trait HyperGraph<N, E>: RawHyperGraph<N, E> {
    type Adj<N2, E2>;
}

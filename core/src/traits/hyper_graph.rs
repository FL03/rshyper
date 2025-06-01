/*
    Appellation: hgraph <module>
    Contrib: @FL03
*/
use crate::Weight;
use crate::index::{EdgeId, RawIndex, VertexId};

pub trait RawNode<T> {
    type Idx: RawIndex;

    private!();

    /// returns an immutable reference to the node index
    fn index(&self) -> &VertexId<Self::Idx>;
    /// returns an immutable reference to the node data
    fn weight(&self) -> Weight<&T>;
}

pub trait RawEdge<T> {
    type Idx: RawIndex;
    type Points<_T>;

    private!();

    /// returns an immutable reference to the edge index
    fn index(&self) -> &EdgeId<Self::Idx>;
    /// Returns an immutable reference to the edge data.
    fn vertices(&self) -> &Self::Points<VertexId<Self::Idx>>;

    /// Returns the index of the edge.
    fn weight(&self) -> Weight<&T>;
}

pub trait RawHyperGraph<N, E> {
    type Idx: RawIndex;

    fn insert_node(&mut self, weight: N) -> VertexId<Self::Idx>;
    /// inserts a new vertex
    fn insert_vertex(&mut self) -> VertexId<Self::Idx>;
}

/// [`HyperGraph`] is a trait that defines the basic operations for a hypergraph data structure.
pub trait HyperGraph<N, E>: RawHyperGraph<N, E> {
    type Adj<N2, E2>;
}

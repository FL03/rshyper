/*
    Appellation: hgraph <module>
    Contrib: @FL03
*/
use crate::index::{EdgeId, RawIndex, VertexId};
use crate::{GraphKind, HyperNode, Weight};

/// [`RawHyperGraph`] is a trait that defines the basic operations for a hypergraph data
/// structure.
pub trait RawHyperGraph<N, E> {
    type Idx: RawIndex;
    type Kind: GraphKind;
}

pub trait HyperGraph<N, E>: RawHyperGraph<N, E> {
    /// given an iterable of vertex indices, add an edge to the graph and return its index
    fn add_edge<I>(&mut self, iter: I) -> crate::Result<EdgeId<Self::Idx>>
    where
        I: IntoIterator<Item = VertexId<Self::Idx>>,
        E: Default,
    {
        self.add_surface(iter, Default::default())
    }
    /// given an iterable of vertex indices and a weight, add an edge to the graph and return its index
    fn add_surface<I>(&mut self, iter: I, weight: Weight<E>) -> crate::Result<EdgeId<Self::Idx>>
    where
        I: IntoIterator<Item = VertexId<Self::Idx>>;
    /// add a new node to the graph with the given weight and return its index
    fn add_node(&mut self, weight: N) -> crate::Result<VertexId<Self::Idx>>;
    /// add a new default node to the graph and return its index
    fn add_vertex(&mut self) -> crate::Result<VertexId<Self::Idx>>
    where
        N: Default,
    {
        self.add_node(N::default())
    }

    fn get_edge_vertices<S>(&self, index: &EdgeId<Self::Idx>) -> crate::Result<S>
    where
        for<'a> S: core::iter::FromIterator<&'a VertexId<Self::Idx>>;

    fn get_node(&self, index: &VertexId<Self::Idx>) -> crate::Result<&HyperNode<N, Self::Idx>>;

    fn get_facet(&self, index: &EdgeId<Self::Idx>) -> crate::Result<&Weight<E>>;

    fn contains_edge(&self, index: &EdgeId<Self::Idx>) -> bool;

    fn contains_node(&self, index: &VertexId<Self::Idx>) -> bool;
}

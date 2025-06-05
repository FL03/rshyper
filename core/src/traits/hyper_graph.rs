/*
    Appellation: hgraph <module>
    Contrib: @FL03
*/
use crate::edge::{RawEdge, RawFacet};
use crate::index::{EdgeId, VertexId};
use crate::node::RawNode;
use crate::{GraphAttributes, Weight};

/// [`RawHyperGraph`] is a trait that defines the basic operations for a hypergraph data
/// structure.
pub trait RawHyperGraph<A>
where
    A: GraphAttributes,
{
    type Edge<E>: RawFacet<E, Idx = A::Idx, Kind = A::Kind>;
    type Node<N>: RawNode<N, Idx = A::Idx>;
}

pub trait HyperGraph<N, E, A>: RawHyperGraph<A>
where
    A: GraphAttributes,
{
    /// given an iterable of vertex indices, add an edge to the graph and return its index
    fn add_edge<I>(&mut self, iter: I) -> crate::Result<EdgeId<A::Idx>>
    where
        I: IntoIterator<Item = VertexId<A::Idx>>,
        E: Default,
    {
        self.add_surface(iter, Default::default())
    }
    /// given an iterable of vertex indices and a weight, add an edge to the graph and return its index
    fn add_surface<I>(&mut self, iter: I, weight: Weight<E>) -> crate::Result<EdgeId<A::Idx>>
    where
        I: IntoIterator<Item = VertexId<A::Idx>>;
    /// add a new node to the graph with the given weight and return its index
    fn add_node(&mut self, weight: N) -> crate::Result<VertexId<A::Idx>>;
    /// add a new default node to the graph and return its index
    fn add_vertex(&mut self) -> crate::Result<VertexId<A::Idx>>
    where
        N: Default,
    {
        self.add_node(N::default())
    }
    /// returns the vertices of the edge with the given index
    fn get_edge_vertices(
        &self,
        index: &EdgeId<A::Idx>,
    ) -> crate::Result<&<Self::Edge<E> as RawEdge>::Store>;
    /// returns a reference to the node with the given index
    fn get_edge_weight(&self, index: &EdgeId<A::Idx>) -> crate::Result<&Weight<E>>;

    fn get_surface(&self, index: &EdgeId<A::Idx>) -> crate::Result<&Self::Edge<E>>;
    /// returns a reference to the node with the given index
    fn get_node(&self, index: &VertexId<A::Idx>) -> crate::Result<&Self::Node<N>>;
    /// returns true if the graph contains the edge with the given index
    fn contains_edge(&self, index: &EdgeId<A::Idx>) -> bool;
    /// returns true if the graph contains the node with the given index
    fn contains_node(&self, index: &VertexId<A::Idx>) -> bool;
    /// returns an iterator over all edges that contain the given node
    fn find_edges_with_node(
        &self,
        index: &VertexId<A::Idx>,
    ) -> crate::Result<impl Iterator<Item = EdgeId<A::Idx>>>;
}

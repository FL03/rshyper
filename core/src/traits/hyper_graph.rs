/*
    Appellation: hgraph <module>
    Contrib: @FL03
*/
use crate::edge::{RawEdge, RawFacet};
use crate::idx::{EdgeId, VertexId};
use crate::node::RawNode;
use crate::{GraphAttributes, Weight};

pub trait BinaryEdge: RawEdge {
    fn lhs(&self) -> &VertexId<Self::Index>;
    fn rhs(&self) -> &VertexId<Self::Index>;
}

/// [`RawHyperGraph`] is a trait that defines the basic operations for a hypergraph data
/// structure.
pub trait RawHyperGraph<A>
where
    A: GraphAttributes,
{
    type Edge<E>: RawFacet<E, Index = A::Ix, Kind = A::Kind>;
    type Node<N>: RawNode<N, Key = A::Ix>;
}

pub trait HyperGraph<N, E, A>: RawHyperGraph<A>
where
    A: GraphAttributes,
{
    /// given an iterable of vertex indices, add an edge to the graph and return its index
    fn add_edge<I>(&mut self, iter: I) -> crate::Result<EdgeId<A::Ix>>
    where
        I: IntoIterator<Item = VertexId<A::Ix>>,
        E: Default,
    {
        self.add_surface(iter, Default::default())
    }
    /// given an iterable of vertex indices and a weight, add an edge to the graph and return its index
    fn add_surface<I>(&mut self, iter: I, weight: Weight<E>) -> crate::Result<EdgeId<A::Ix>>
    where
        I: IntoIterator<Item = VertexId<A::Ix>>;
    /// add a new node to the graph with the given weight and return its index
    fn add_node(&mut self, weight: Weight<N>) -> crate::Result<VertexId<A::Ix>>;
    /// add a new default node to the graph and return its index
    fn add_vertex(&mut self) -> crate::Result<VertexId<A::Ix>>
    where
        N: Default,
    {
        self.add_node(Default::default())
    }
    /// returns the vertices of the edge with the given index
    fn get_edge_vertices(
        &self,
        index: &EdgeId<A::Ix>,
    ) -> crate::Result<&<Self::Edge<E> as RawEdge>::Store>;
    /// returns a mutable reference to the vertices of the edge with the given index
    fn get_edge_vertices_mut(
        &mut self,
        index: &EdgeId<A::Ix>,
    ) -> crate::Result<&mut <Self::Edge<E> as RawEdge>::Store>;
    /// returns a reference to the weight of the edge with the given index
    fn get_edge_weight(&self, index: &EdgeId<A::Ix>) -> crate::Result<&Weight<E>> {
        self.get_surface(index).map(|edge| edge.weight())
    }
    /// returns a mutable reference to the weight of the edge with the given index
    fn get_edge_weight_mut(&mut self, index: &EdgeId<A::Ix>) -> crate::Result<&mut Weight<E>> {
        self.get_surface_mut(index).map(|edge| edge.weight_mut())
    }
    /// returns an immutable reference to the edge with the given index
    fn get_surface(&self, index: &EdgeId<A::Ix>) -> crate::Result<&Self::Edge<E>>;
    /// returns a mutable reference to the edge with the given index
    fn get_surface_mut(&mut self, index: &EdgeId<A::Ix>) -> crate::Result<&mut Self::Edge<E>>;
    /// returns a reference to the node with the given index
    fn get_node(&self, index: &VertexId<A::Ix>) -> crate::Result<&Self::Node<N>>;
    /// returns a mutable reference to the node with the given index
    fn get_node_mut(&mut self, index: &VertexId<A::Ix>) -> crate::Result<&mut Self::Node<N>>;
    /// returns true if the graph contains the edge with the given index
    fn contains_edge(&self, index: &EdgeId<A::Ix>) -> bool;
    /// returns true if the graph contains the node with the given index
    fn contains_node(&self, index: &VertexId<A::Ix>) -> bool;
    /// returns an iterator over all edges that contain the given node
    fn find_edges_with_node(
        &self,
        index: &VertexId<A::Ix>,
    ) -> crate::Result<impl Iterator<Item = EdgeId<A::Ix>>>;
}

/*
 ************* Implementations *************
*/
use crate::GraphType;
use crate::edge::{Edge, Surface};
use crate::idx::RawIndex;

impl<I, K> BinaryEdge for Edge<[VertexId<I>; 2], K, I>
where
    I: RawIndex,
    K: GraphType,
{
    fn lhs(&self) -> &VertexId<I> {
        &self.points()[0]
    }

    fn rhs(&self) -> &VertexId<I> {
        &self.points()[1]
    }
}

impl<E, I, K> BinaryEdge for Surface<E, [VertexId<I>; 2], K, I>
where
    I: RawIndex,
    K: GraphType,
{
    fn lhs(&self) -> &VertexId<I> {
        &self.points()[0]
    }

    fn rhs(&self) -> &VertexId<I> {
        &self.points()[1]
    }
}

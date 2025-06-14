/*
    Appellation: hgraph <module>
    Contrib: @FL03
*/
use crate::edge::{BinaryEdge, RawEdge, RawFacet};
use crate::idx::{EdgeId, VertexId};
use crate::node::RawNode;
use crate::{GraphAttributes, Weight};

/// [`RawHyperGraph`] is a trait that defines the basic operations for a hypergraph data
/// structure.
pub trait RawHyperGraph<A>
where
    A: GraphAttributes,
{
    type Edge<E>: RawFacet<E, Index = A::Ix, Kind = A::Kind>;
    type Node<N>: RawNode<N, Key = A::Ix>;
}
/// The [`HyperGraph`] trait directly extends the [`RawHyperGraph`] trait to provide additional
/// utilities and constructors for implementors while establishing a more robust interface for
/// hypergraphs. This trait is designed to abstract the basic behaviour of hypergraphs,
/// enabling the generalization of implements algorithms and operators.
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

/// The [`StdGraph`] is used to denotes instances in-which the hypergraph contains binary edges
/// meaning that each edge is composed of exactly two vertices.
pub trait StdGraph<N, E, A>: RawHyperGraph<A>
where
    A: GraphAttributes,
    Self::Edge<E>: BinaryEdge,
{
}

/*
 ************* Implementations *************
*/

impl<N, E, A, H> StdGraph<N, E, A> for H
where
    A: GraphAttributes,
    N: Default,
    E: Default,
    H: HyperGraph<N, E, A>,
    H::Edge<E>: BinaryEdge,
{
}

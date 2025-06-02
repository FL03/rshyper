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
        I: IntoIterator<Item = VertexId<Self::Idx>>;
    /// given an iterable of vertex indices and a weight, add an edge to the graph and return its index
    fn add_edge_with_weight<I>(
        &mut self,
        iter: I,
        weight: Weight<E>,
    ) -> crate::Result<EdgeId<Self::Idx>>
    where
        I: IntoIterator<Item = VertexId<Self::Idx>>,
        Self::Idx: Clone,
    {
        // insert the edge with the given vertices
        let edge_index = self.add_edge(iter)?;
        // assign the weight to the edge
        let _prev = self.add_facet(edge_index.clone(), weight)?;
        Ok(edge_index)
    }
    /// add a facet, or weight, to an existing edge and return the previous weight if it exists
    fn add_facet(
        &mut self,
        index: EdgeId<Self::Idx>,
        weight: Weight<E>,
    ) -> crate::Result<Option<Weight<E>>>;
    /// add a new node to the graph with the given weight and return its index
    fn add_node(&mut self, weight: N) -> VertexId<Self::Idx>;
    /// add a new default node to the graph and return its index
    fn add_vertex(&mut self) -> VertexId<Self::Idx>
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

    fn contains_facet(&self, index: &EdgeId<Self::Idx>) -> bool;

    fn contains_node(&self, index: &VertexId<Self::Idx>) -> bool;
}

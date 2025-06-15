/*
    appellation: impl_hyper_graph <module>
    authors: @FL03
*/
use crate::{HashSurface, HyperMap, iter};
use core::hash::{BuildHasher, Hash};
use rshyper_core::idx::{EdgeId, NumIndex, VertexId};
use rshyper_core::prelude::{GraphAttributes, HyperResult, Node, VertexSet, Weight};
use rshyper_core::traits::{HyperGraph, HyperGraphIterEdge, HyperGraphIterNode, RawHyperGraph};

impl<N, E, A, S> RawHyperGraph<A> for HyperMap<N, E, A, S>
where
    A: GraphAttributes,
    S: BuildHasher,
{
    type Node<_N> = Node<_N, A::Ix>;
    type Edge<_E> = HashSurface<_E, A::Kind, A::Ix, S>;
}

impl<N, E, A, S> HyperGraph<N, E, A> for HyperMap<N, E, A, S>
where
    A: GraphAttributes,
    S: BuildHasher + Default,
    A::Ix: NumIndex,
{
    fn add_node(&mut self, weight: Weight<N>) -> HyperResult<VertexId<A::Ix>> {
        self.add_node(weight)
    }

    fn add_surface<I>(&mut self, iter: I, weight: Weight<E>) -> HyperResult<EdgeId<A::Ix>>
    where
        I: IntoIterator<Item = VertexId<A::Ix>>,
    {
        self.add_surface(iter, weight)
    }

    fn get_edge_domain(&self, index: &EdgeId<A::Ix>) -> Option<&VertexSet<A::Ix, S>> {
        self.get_edge_vertices(index).ok()
    }

    fn get_edge_domain_mut(&mut self, index: &EdgeId<A::Ix>) -> Option<&mut VertexSet<A::Ix, S>> {
        self.get_edge_vertices_mut(index).ok()
    }

    fn get_edge_weight(&self, index: &EdgeId<A::Ix>) -> Option<&Weight<E>> {
        self.get_edge_weight(index).ok()
    }

    fn get_edge_weight_mut(&mut self, index: &EdgeId<A::Ix>) -> Option<&mut Weight<E>> {
        self.get_edge_weight_mut(index).ok()
    }

    fn get_node(&self, index: &VertexId<A::Ix>) -> Option<&Node<N, A::Ix>> {
        self.get_node(index).ok()
    }

    fn get_node_mut(&mut self, index: &VertexId<A::Ix>) -> Option<&mut Node<N, A::Ix>> {
        self.get_node_mut(index).ok()
    }

    fn get_surface(&self, index: &EdgeId<A::Ix>) -> Option<&HashSurface<E, A::Kind, A::Ix, S>> {
        self.get_surface(index).ok()
    }

    fn get_surface_mut(
        &mut self,
        index: &EdgeId<A::Ix>,
    ) -> Option<&mut HashSurface<E, A::Kind, A::Ix, S>> {
        self.get_surface_mut(index).ok()
    }

    fn contains_edge(&self, index: &EdgeId<A::Ix>) -> bool {
        self.contains_edge(index)
    }

    fn contains_node(&self, index: &VertexId<A::Ix>) -> bool {
        self.contains_node(index)
    }

    fn find_edges_with_node(&self, index: &VertexId<A::Ix>) -> Vec<EdgeId<A::Ix>> {
        self.find_edges_with_node(index).unwrap_or_default()
    }
}

impl<N, E, A, S> HyperGraphIterNode<N, E, A> for HyperMap<N, E, A, S>
where
    A: GraphAttributes,
    S: BuildHasher + Default,
    E: Eq + Hash,
    N: Eq + Hash,
    A::Ix: NumIndex,
{
    type Nodes<'a>
        = iter::NodeIter<'a, N, A::Ix>
    where
        Self: 'a,
        Self::Node<N>: 'a;
    type Verts<'a>
        = iter::Points<'a, N, A::Ix>
    where
        Self: 'a;

    fn iter_nodes(&self) -> Self::Nodes<'_> {
        self.node_iter()
    }

    fn vertices(&self) -> Self::Verts<'_> {
        self.points()
    }
}

impl<N, E, A, S> HyperGraphIterEdge<N, E, A> for HyperMap<N, E, A, S>
where
    A: GraphAttributes,
    S: BuildHasher + Default,
    E: Eq + Hash,
    N: Eq + Hash,
    A::Ix: NumIndex,
{
    type Surfaces<'a>
        = iter::SurfaceIter<'a, E, A::Kind, A::Ix, S>
    where
        Self: 'a,
        Self::Edge<E>: 'a;

    type Edges<'a>
        = iter::Edges<'a, E, A::Kind, A::Ix, S>
    where
        Self: 'a,
        Self::Edge<E>: 'a;

    fn iter_surfaces(&self) -> Self::Surfaces<'_> {
        self.surface_iter()
    }

    fn edges(&self) -> Self::Edges<'_> {
        self.edges()
    }
}

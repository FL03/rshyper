/*
    appellation: impl_hyper_graph <module>
    authors: @FL03
*/
use crate::{HashEdge, HyperMap, VertexSet, iter};
use core::hash::{BuildHasher, Hash};
use rshyper::error::Result;
use rshyper::idx::{EdgeId, NumIndex, VertexId};
use rshyper::prelude::{GraphProps, Node, Weight};
use rshyper::traits::{HyperGraph, HyperGraphIterEdge, HyperGraphIterNode, RawHyperGraph};

impl<N, E, A, S> RawHyperGraph<A> for HyperMap<N, E, A, S>
where
    A: GraphProps,
    S: BuildHasher,
{
    type Node<_N> = Node<_N, A::Ix>;
    type Edge<_E> = HashEdge<_E, A::Kind, A::Ix, S>;
}

impl<N, E, A, S> HyperGraph<N, E, A> for HyperMap<N, E, A, S>
where
    A: GraphProps,
    S: BuildHasher + Default,
    A::Ix: NumIndex,
{
    fn add_node(&mut self, weight: Weight<N>) -> Result<VertexId<A::Ix>> {
        self.add_node(weight)
    }

    fn add_surface<I>(&mut self, iter: I, weight: Weight<E>) -> Result<EdgeId<A::Ix>>
    where
        I: IntoIterator<Item = VertexId<A::Ix>>,
    {
        self.add_surface(iter, weight)
    }

    fn get_domain(&self, index: &EdgeId<A::Ix>) -> Option<&VertexSet<A::Ix, S>> {
        self.get_domain(index).ok()
    }

    fn get_domain_mut(&mut self, index: &EdgeId<A::Ix>) -> Option<&mut VertexSet<A::Ix, S>> {
        self.get_domain_mut(index).ok()
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

    fn get_edge(&self, index: &EdgeId<A::Ix>) -> Option<&HashEdge<E, A::Kind, A::Ix, S>> {
        self.get_surface(index).ok()
    }

    fn get_edge_mut(
        &mut self,
        index: &EdgeId<A::Ix>,
    ) -> Option<&mut HashEdge<E, A::Kind, A::Ix, S>> {
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
    A: GraphProps,
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
        self.iter_nodes()
    }

    fn vertices(&self) -> Self::Verts<'_> {
        self.points()
    }
}

impl<N, E, A, S> HyperGraphIterEdge<N, E, A> for HyperMap<N, E, A, S>
where
    A: GraphProps,
    S: BuildHasher + Default,
    E: Eq + Hash,
    N: Eq + Hash,
    A::Ix: NumIndex,
{
    type Surfaces<'a>
        = iter::EdgeIter<'a, E, A::Kind, A::Ix, S>
    where
        Self: 'a,
        Self::Edge<E>: 'a;

    type Edges<'a>
        = iter::EdgeKeys<'a, E, A::Kind, A::Ix, S>
    where
        Self: 'a,
        Self::Edge<E>: 'a;

    fn iter_surfaces(&self) -> Self::Surfaces<'_> {
        self.iter_edges()
    }

    fn edges(&self) -> Self::Edges<'_> {
        self.iter_edge_ids()
    }
}

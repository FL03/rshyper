/*
    appellation: impl_ops <module>
    authors: @FL03
*/
use crate::hash_graph::HashGraph;
use rshyper_core::{EdgeId, HyperNode, NumIndex, VertexId};

impl<N, E, Idx> core::ops::Index<&EdgeId<Idx>> for HashGraph<N, E, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    Idx: NumIndex,
{
    type Output = E;

    fn index(&self, index: &EdgeId<Idx>) -> &Self::Output {
        self.get_facet(index).expect("Edge not found")
    }
}

impl<N, E, Idx> core::ops::IndexMut<&EdgeId<Idx>> for HashGraph<N, E, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    Idx: NumIndex,
{
    fn index_mut(&mut self, index: &EdgeId<Idx>) -> &mut Self::Output {
        self.get_facet_mut(index).expect("Edge not found")
    }
}

impl<N, E, Idx> core::ops::Index<&VertexId<Idx>> for HashGraph<N, E, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    Idx: NumIndex,
{
    type Output = HyperNode<N, Idx>;

    fn index(&self, index: &VertexId<Idx>) -> &Self::Output {
        self.get_vertex_weight(index).expect("Node not found")
    }
}

impl<N, E, Idx> core::ops::IndexMut<&VertexId<Idx>> for HashGraph<N, E, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    Idx: NumIndex,
{
    fn index_mut(&mut self, index: &VertexId<Idx>) -> &mut Self::Output {
        self.get_vertex_weight_mut(index).expect("Node not found")
    }
}

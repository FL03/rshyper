/*
    appellation: impl_ops <module>
    authors: @FL03
*/
use crate::hash_graph::graph::HashGraph;
use crate::{EdgeId, Node, VertexId};

impl<N, E> core::ops::Index<EdgeId> for HashGraph<N, E>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
{
    type Output = E;

    fn index(&self, index: EdgeId) -> &Self::Output {
        self.get_facet(index).expect("Edge not found")
    }
}

impl<N, E> core::ops::IndexMut<EdgeId> for HashGraph<N, E>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
{
    fn index_mut(&mut self, index: EdgeId) -> &mut Self::Output {
        self.get_facet_mut(index).expect("Edge not found")
    }
}

impl<N, E> core::ops::Index<VertexId> for HashGraph<N, E>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
{
    type Output = Node<N>;

    fn index(&self, index: VertexId) -> &Self::Output {
        self.get_vertex_weight(index).expect("Node not found")
    }
}

impl<N, E> core::ops::IndexMut<VertexId> for HashGraph<N, E>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
{
    fn index_mut(&mut self, index: VertexId) -> &mut Self::Output {
        self.get_vertex_weight_mut(index).expect("Node not found")
    }
}

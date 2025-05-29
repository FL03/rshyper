use crate::hash_graph::graph::HashGraph;
use crate::{Node, VertexId};

impl<N, E> core::ops::Index<VertexId> for HashGraph<N, E>
where
    N: core::hash::Hash + Eq,
    E: core::hash::Hash + Eq,
{
    type Output = Node<N>;

    fn index(&self, index: VertexId) -> &Self::Output {
        self.get_vertex_weight(index).expect("Node not found")
    }
}

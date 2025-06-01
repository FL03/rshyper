/*
    appellation: impl_ops <module>
    authors: @FL03
*/
use crate::binary_graph::BinaryGraph;
use crate::index::{EdgeId, RawIndex, VertexId};
use rshyper_core::HyperNode;

impl<N, E, Idx> BinaryGraph<N, E, Idx>
where
    N: Ord,
    E: Ord,
    Idx: RawIndex + Ord,
{
}

impl<N, E, Idx> core::ops::Index<&EdgeId<Idx>> for BinaryGraph<N, E, Idx>
where
    N: Ord,
    E: Ord,
    Idx: RawIndex + Ord,
{
    type Output = E;

    fn index(&self, index: &EdgeId<Idx>) -> &Self::Output {
        self.facets()
            .get(index)
            .expect("EdgeId not found in connections")
    }
}

impl<N, E, Idx> core::ops::Index<&VertexId<Idx>> for BinaryGraph<N, E, Idx>
where
    N: Ord,
    E: Ord,
    Idx: RawIndex + Ord,
{
    type Output = HyperNode<N, Idx>;

    fn index(&self, index: &VertexId<Idx>) -> &Self::Output {
        self.nodes()
            .get(index)
            .expect("VertexId not found in connections")
    }
}

impl<N, E, Idx> core::ops::IndexMut<&EdgeId<Idx>> for BinaryGraph<N, E, Idx>
where
    N: Ord,
    E: Ord,
    Idx: RawIndex + Ord,
{
    fn index_mut(&mut self, index: &EdgeId<Idx>) -> &mut Self::Output {
        self.facets_mut()
            .get_mut(index)
            .expect("EdgeId not found in connections")
    }
}

impl<N, E, Idx> core::ops::IndexMut<&VertexId<Idx>> for BinaryGraph<N, E, Idx>
where
    N: Ord,
    E: Ord,
    Idx: RawIndex + Ord,
{
    fn index_mut(&mut self, index: &VertexId<Idx>) -> &mut Self::Output {
        self.nodes_mut()
            .get_mut(index)
            .expect("VertexId not found in connections")
    }
}

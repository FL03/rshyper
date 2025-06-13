/*
    appellation: impl_ops <module>
    authors: @FL03
*/
use crate::binary_graph::{BTreeFacet, BinaryGraph};
use crate::idx::{EdgeId, RawIndex, VertexId};
use crate::{GraphAttributes, GraphType};
use core::ops::{Index, IndexMut};
use rshyper_core::Node;

impl<N, E, A, K, Idx> BinaryGraph<N, E, A>
where
    N: Ord,
    E: Ord,
    A: GraphAttributes<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: RawIndex + Ord,
{
}

impl<N, E, A, K, Idx> Index<&VertexId<Idx>> for BinaryGraph<N, E, A>
where
    N: Ord,
    E: Ord,
    A: GraphAttributes<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: RawIndex + Ord,
{
    type Output = Node<N, Idx>;

    fn index(&self, index: &VertexId<Idx>) -> &Self::Output {
        self.get_node(index)
            .expect("No node found for the given index")
    }
}

impl<N, E, A, K, Idx> IndexMut<&VertexId<Idx>> for BinaryGraph<N, E, A>
where
    N: Ord,
    E: Ord,
    A: GraphAttributes<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: RawIndex + Ord,
{
    fn index_mut(&mut self, index: &VertexId<Idx>) -> &mut Self::Output {
        self.get_node_mut(index)
            .expect("VertexId not found in connections")
    }
}

impl<N, E, A, K, Idx> Index<&EdgeId<Idx>> for BinaryGraph<N, E, A>
where
    N: Ord,
    E: Ord,
    A: GraphAttributes<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: RawIndex + Ord,
{
    type Output = BTreeFacet<E, K, Idx>;

    fn index(&self, index: &EdgeId<Idx>) -> &Self::Output {
        self.get_surface(index)
            .expect("No edge found for the given index")
    }
}

impl<N, E, A, K, Idx> IndexMut<&EdgeId<Idx>> for BinaryGraph<N, E, A>
where
    N: Ord,
    E: Ord,
    A: GraphAttributes<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: RawIndex + Ord,
{
    fn index_mut(&mut self, index: &EdgeId<Idx>) -> &mut Self::Output {
        self.get_surface_mut(index)
            .expect("No edge found for the given index")
    }
}

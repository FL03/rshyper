/*
    appellation: impl_repr <module>
    authors: @FL03
*/
use crate::binary_graph::BinaryGraph;
use crate::index::RawIndex;

impl<N, E, Idx> BinaryGraph<N, E, Idx>
where
    N: Ord,
    E: Ord,
    Idx: RawIndex + Ord,
{
}

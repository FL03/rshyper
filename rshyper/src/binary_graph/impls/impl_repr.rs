/*
    appellation: impl_repr <module>
    authors: @FL03
*/
use crate::binary_graph::BinaryGraph;
use rshyper_core::index::RawIndex;
use rshyper_core::{Directed, GraphAttributes, Undirected};

impl<N, E, A, Idx> BinaryGraph<N, E, A>
where
    N: Ord,
    E: Ord,
    A: GraphAttributes<Idx = Idx, Kind = Directed>,
    Idx: RawIndex + Ord,
{
    /// initialize a new, empty directed binary graph
    pub fn directed() -> Self
    where
        Idx: Default,
    {
        BinaryGraph::new()
    }
}

impl<N, E, A, Idx> BinaryGraph<N, E, A>
where
    N: Ord,
    E: Ord,
    A: GraphAttributes<Idx = Idx, Kind = Undirected>,
    Idx: RawIndex + Ord,
{
    /// initialize a new, empty undirected binary graph
    pub fn undirected() -> Self
    where
        Idx: Default,
    {
        BinaryGraph::new()
    }
}

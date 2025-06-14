/*
    appellation: base <module>
    authors: @FL03
*/
use crate::edge::Edge;
use crate::{GraphType, RawIndex, RawStore, VertexId};

/// returns a new [`Edge`] from the given iterator of vertex ids
pub fn edge<I, S, K, Idx>(iter: I) -> Edge<S, K, Idx>
where
    I: IntoIterator<Item = S::Item>,
    S: RawStore<Item = VertexId<Idx>>,
    K: GraphType,
    Idx: RawIndex,
    Edge<S, K, Idx>: FromIterator<S::Item>,
{
    Edge::from_iter(iter)
}

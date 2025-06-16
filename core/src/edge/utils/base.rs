/*
    appellation: base <module>
    authors: @FL03
*/
use crate::edge::EdgeLayout;
use crate::{GraphType, RawDomain, RawIndex, VertexId};

/// returns a new [`Edge`] from the given iterator of vertex ids
pub fn edge<I, S, K, Idx>(iter: I) -> EdgeLayout<S, K, Idx>
where
    I: IntoIterator<Item = S::Item>,
    S: RawDomain<Item = VertexId<Idx>>,
    K: GraphType,
    Idx: RawIndex,
    EdgeLayout<S, K, Idx>: FromIterator<S::Item>,
{
    EdgeLayout::from_iter(iter)
}

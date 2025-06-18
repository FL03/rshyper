/*
    appellation: base <module>
    authors: @FL03
*/
use crate::edge::Edge;
use crate::rel::Link;
use crate::{GraphType, RawDomain, RawIndex, VertexId};

/// returns a new [`Edge`] from the given iterator of vertex ids
pub fn edge<I, T, S, K, Idx>(iter: I) -> Edge<T, S, K, Idx>
where
    I: IntoIterator<Item = S::Item>,
    S: RawDomain<Item = VertexId<Idx>>,
    T: Default,
    K: GraphType,
    Idx: RawIndex,
    Link<S, K, Idx>: FromIterator<S::Item>,
{
    let rel = Link::from_iter(iter);
    Edge::from_link(rel)
}

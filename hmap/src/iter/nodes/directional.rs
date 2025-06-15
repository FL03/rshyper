use core::hash::Hash;
use rshyper_core::Node;
use rshyper_core::idx::{RawIndex, VertexId};
use std::collections::hash_map;

#[doc(hidden)]
/// returns an iterator that starts at the "first" vertex and follows the path until the end
pub struct DirectedNodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    #[allow(dead_code)]
    pub(crate) iter: hash_map::Iter<'a, VertexId<Idx>, Node<N, Idx>>,
}

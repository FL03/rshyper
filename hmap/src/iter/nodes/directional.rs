/*
    appellation: directional <nodes>
    authors: @FL03
*/
use core::hash::Hash;
use hashbrown::hash_map;
use rshyper_core::idx::{RawIndex, VertexId};
use rshyper_core::node::Node;

#[doc(hidden)]
/// returns an iterator that starts at the "first" vertex and follows the path until the end
pub struct DirectedNodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    #[allow(dead_code)]
    pub(crate) iter: hash_map::Iter<'a, VertexId<Idx>, Node<N, Idx>>,
}

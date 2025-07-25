/*
    appellation: directional <nodes>
    authors: @FL03
*/
use core::hash::Hash;
use hashbrown::hash_map;
use rshyper::idx::{RawIndex, VertexId};
use rshyper::node::Node;

#[doc(hidden)]
/// returns an iterator that starts at the "first" vertex and follows the path until the end
pub struct DirectedNodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    #[allow(dead_code)]
    pub(crate) iter: hash_map::Iter<'a, VertexId<Idx>, Node<N, Idx>>,
}

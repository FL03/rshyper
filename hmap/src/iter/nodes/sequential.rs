/*
    appellation: seq <module>
    authors: @FL03
*/
//! this module implements sequential iterator for the [`HyperMap`](crate::HyperMap)
//! which iterates over the nodes in the hypergraph in an ordered, sequential manner.
use super::NodeIterValues;
use core::hash::Hash;
use rshyper_core::idx::{RawIndex, VertexId};
use rshyper_core::node::Node;

/// [`SeqNodeIter`] is an iterator over the nodes of a hypergraph, yielding elements according
/// to the order in-which they were inserted.
pub struct SeqNodeIter<'a, N, Idx>
where
    N: 'a,
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) nodes: NodeIterValues<'a, N, Idx>,
    pub(crate) verts: core::slice::Iter<'a, VertexId<Idx>>,
}

/*
 ************* Implementations *************
*/

impl<'a, N, Idx> Iterator for SeqNodeIter<'a, N, Idx>
where
    N: 'a,
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a Node<N, Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.verts.next() {
            // Find the node with the matching id in the nodes iterator
            let node = self.nodes.find(|node| node.id() == next)?;
            // Return the found node
            return Some(node);
        }
        None
    }
}

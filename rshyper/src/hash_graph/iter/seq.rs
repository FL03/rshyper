

use crate::idx::RawIndex;
use crate::node::Node;
use core::hash::Hash;

pub struct SeqNodeIter<'a, N, Idx>
where
    N: 'a,
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: core::slice::Iter<'a, Node<N, Idx>>,
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
        self.iter.next()
    }
}

/*
    appellation: seq <module>
    authors: @FL03
*/
use core::hash::Hash;
use rshyper_core::idx::RawIndex;
use rshyper_core::node::Node;

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

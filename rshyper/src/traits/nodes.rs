/*
    Appellation: nodes <module>
    Contrib: @FL03
*/

/// A trait denoting a node within the hypergraph.
pub trait HyperNode<Idx> {
    fn index(&self) -> &crate::Index<Idx>;
}

/// Extends the base [HyperNode] trait with the [core::cmp::Eq] and [core::hash::Hash] traits
/// for use with hash-related structures.
pub trait HashNode<Idx>: HyperNode<Idx> + core::cmp::Eq + core::hash::Hash {}

pub trait Weighted<Idx>: HyperNode<Idx> {
    type Data;

    fn weight(&self) -> &Self::Data;
}

/*
 ************* Implementations *************
*/
impl<T, Idx> HyperNode<Idx> for T
where
    T: crate::Indexable<Idx>,
{
    fn index(&self) -> &crate::Index<Idx> {
        self.index()
    }
}
impl<T, Idx> HashNode<Idx> for T where T: HyperNode<Idx> + core::cmp::Eq + core::hash::Hash {}

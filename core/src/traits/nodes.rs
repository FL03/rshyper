/*
    Appellation: nodes <module>
    Contrib: @FL03
*/
use crate::VertexId;

/// A trait denoting a node within the hypergraph.
pub trait HyperNode<Idx> {
    fn index(&self) -> &VertexId<Idx>;
}

/// Extends the base [HyperNode] trait with the [`Eq`] and [`Hash`](core::hash::Hash) traits
/// for use with hash-related structures.
pub trait HashNode<Idx>: HyperNode<Idx> + Eq + core::hash::Hash {
    private!();
}


pub trait Weighted {
    type Data;

    fn weight(&self) -> &Self::Data;
}

/*
 ************* Implementations *************
*/
impl<T, Idx> HashNode<Idx> for T
where
    T: HyperNode<Idx> + Eq + core::hash::Hash,
{
    seal!();
}

impl<T, Idx> HyperNode<Idx> for T
where
    T: core::borrow::Borrow<VertexId<Idx>>,
{
    fn index(&self) -> &VertexId<Idx> {
        self.borrow()
    }
}

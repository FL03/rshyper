/*
    Appellation: nodes <module>
    Contrib: @FL03
*/
use crate::{VertexId, Weight};

/// A trait denoting a node within the hypergraph.
pub trait HyperNode<Idx> {
    fn index(&self) -> &VertexId<Idx>;
}

/// Extends the base [HyperNode] trait with the [`Eq`] and [`Hash`](core::hash::Hash) traits
/// for use with hash-related structures.
pub trait HashNode<Idx>: HyperNode<Idx> + Eq + core::hash::Hash {
    private!();
}

pub trait Weighted<T> {
    fn weight(&self) -> &Weight<T>;

    fn weight_mut(&mut self) -> &mut Weight<T>;
}

/*
 ************* Implementations *************
*/
impl<T> Weighted<T> for T
where
    T: AsRef<Weight<T>> + AsMut<Weight<T>>,
{
    fn weight(&self) -> &Weight<T> {
        self.as_ref()
    }

    fn weight_mut(&mut self) -> &mut Weight<T> {
        self.as_mut()
    }
}

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

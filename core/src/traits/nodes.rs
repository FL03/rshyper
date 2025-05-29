/*
    Appellation: nodes <module>
    Contrib: @FL03
*/
use crate::VertexId;

/// A trait denoting a node within the hypergraph.
pub trait HyperNode<Idx> {
    fn index(&self) -> &VertexId<Idx>;
}

pub trait Weighted<Idx>: HyperNode<Idx> {
    type Data;

    fn weight(&self) -> &Self::Data;
}

/*
 ************* Implementations *************
*/
impl<T, Idx> HyperNode<Idx> for T
where
    T: core::borrow::Borrow<VertexId<Idx>>,
{
    fn index(&self) -> &VertexId<Idx> {
        self.borrow()
    }
}



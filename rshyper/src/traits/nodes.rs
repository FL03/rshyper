/*
    Appellation: nodes <module>
    Contrib: @FL03
*/

/// A trait denoting a node within the hypergraph.
pub trait HyperNode<Idx> {
    fn index(&self) -> &crate::VertexId<Idx>;
}



pub trait Weighted<Idx>: HyperNode<Idx> {
    type Data;

    fn weight(&self) -> &Self::Data;
}

/*
 ************* Implementations *************
*/
impl<T, Idx> HyperNode<Idx> for T where T: core::borrow::Borrow<crate::VertexId<Idx>> {
    fn index(&self) -> &crate::VertexId<Idx> {
        self.borrow()
    }
}


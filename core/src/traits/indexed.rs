/*
    Appellation: indexed <module>
    Contrib: @FL03
*/

/// This trait is used to denote a type that is aware of its own index.
pub trait Indexed<T> {
    type Idx<I>;

    /// Returns the index of the node.
    fn index(&self) -> &Self::Idx<T>;
}


/*
    ************* Implementations *************
*/
use crate::VertexId;
use crate::node::Node;

impl<T> Indexed<T> for VertexId<T> {
    type Idx<I> = VertexId<I>;

    fn index(&self) -> &Self::Idx<T> {
        self
    }
}

impl<T, Idx> Indexed<Idx> for Node<T, Idx> {
    type Idx<I> = VertexId<I>;

    fn index(&self) -> &Self::Idx<Idx> {
        &self.index
    }
}

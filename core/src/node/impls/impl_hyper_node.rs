/*
    appellation: impl_hyper_node <module>
    authors: @FL03
*/
use crate::idx::{RawIndex, VertexId};
use crate::node::Node;
use crate::weight::Weight;

impl<T, Idx> Node<core::marker::PhantomData<T>, Idx>
where
    Idx: RawIndex,
{
    /// returns a new, weightless node with the given index
    pub const fn weightless(id: VertexId<Idx>) -> Self {
        Self::new(id, core::marker::PhantomData::<T>)
    }
    /// initialize the weight of the node with the given value
    pub fn init_weight<F>(self, init: F) -> Node<T, Idx>
    where
        F: FnOnce() -> T,
    {
        Node::new(self.id, init())
    }
}

impl<T, Idx> AsRef<Weight<T>> for Node<T, Idx>
where
    Idx: RawIndex,
{
    fn as_ref(&self) -> &Weight<T> {
        self.weight()
    }
}

impl<T, Idx> AsMut<Weight<T>> for Node<T, Idx>
where
    Idx: RawIndex,
{
    fn as_mut(&mut self) -> &mut Weight<T> {
        self.weight_mut()
    }
}

impl<T, Idx> core::borrow::Borrow<VertexId<Idx>> for Node<T, Idx>
where
    Idx: RawIndex,
{
    fn borrow(&self) -> &VertexId<Idx> {
        self.id()
    }
}

impl<T, Idx> core::borrow::Borrow<Weight<T>> for Node<T, Idx>
where
    Idx: RawIndex,
{
    fn borrow(&self) -> &Weight<T> {
        self.weight()
    }
}

impl<T, Idx> core::borrow::BorrowMut<Weight<T>> for Node<T, Idx>
where
    Idx: RawIndex,
{
    fn borrow_mut(&mut self) -> &mut Weight<T> {
        self.weight_mut()
    }
}

impl<T, Idx> core::ops::Deref for Node<T, Idx>
where
    Idx: RawIndex,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.weight()
    }
}

impl<T, Idx> core::ops::DerefMut for Node<T, Idx>
where
    Idx: RawIndex,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.weight_mut()
    }
}

impl<T, Idx> From<Weight<T>> for Node<T, Idx>
where
    Idx: Default + RawIndex,
{
    fn from(weight: Weight<T>) -> Self {
        Node::from_weight(weight)
    }
}

impl<T, Idx> From<VertexId<Idx>> for Node<T, Idx>
where
    Idx: RawIndex,
    T: Default,
{
    fn from(index: VertexId<Idx>) -> Self {
        Node::from_id(index)
    }
}

impl<T, Idx> From<(VertexId<Idx>, Weight<T>)> for Node<T, Idx>
where
    Idx: RawIndex,
{
    fn from((index, Weight(weight)): (VertexId<Idx>, Weight<T>)) -> Self {
        Node::new(index, weight)
    }
}

impl<T, Idx> From<Node<T, Idx>> for (VertexId<Idx>, Weight<T>)
where
    Idx: RawIndex,
{
    fn from(node: Node<T, Idx>) -> Self {
        (node.id, node.weight)
    }
}

impl<T, Idx> From<Node<T, Idx>> for VertexId<Idx>
where
    Idx: RawIndex,
{
    fn from(node: Node<T, Idx>) -> Self {
        node.id
    }
}

/*
    appellation: impl_hyper_node <module>
    authors: @FL03
*/
use crate::Weight;
use crate::cmp::HyperNode;
use crate::index::{RawIndex, VertexId};

impl<T, Idx> AsRef<Weight<T>> for HyperNode<T, Idx>
where
    Idx: RawIndex,
{
    fn as_ref(&self) -> &Weight<T> {
        self.weight()
    }
}

impl<T, Idx> AsMut<Weight<T>> for HyperNode<T, Idx>
where
    Idx: RawIndex,
{
    fn as_mut(&mut self) -> &mut Weight<T> {
        self.weight_mut()
    }
}

impl<T, Idx> core::borrow::Borrow<VertexId<Idx>> for HyperNode<T, Idx>
where
    Idx: RawIndex,
{
    fn borrow(&self) -> &VertexId<Idx> {
        self.index()
    }
}

impl<T, Idx> core::borrow::Borrow<Weight<T>> for HyperNode<T, Idx>
where
    Idx: RawIndex,
{
    fn borrow(&self) -> &Weight<T> {
        self.weight()
    }
}

impl<T, Idx> core::borrow::BorrowMut<Weight<T>> for HyperNode<T, Idx>
where
    Idx: RawIndex,
{
    fn borrow_mut(&mut self) -> &mut Weight<T> {
        self.weight_mut()
    }
}

impl<T, Idx> core::ops::Deref for HyperNode<T, Idx>
where
    Idx: RawIndex,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.weight()
    }
}

impl<T, Idx> core::ops::DerefMut for HyperNode<T, Idx>
where
    Idx: RawIndex,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.weight_mut()
    }
}

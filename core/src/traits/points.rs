/*
    appellation: points <module>
    authors: @FL03
*/
use crate::Weight;
use crate::index::{RawIndex, VertexId};

/// A [`RawPoint`] is used to defines the base representation of any given point within a
/// hypergraph.
pub trait RawPoint {
    type Key: RawIndex;

    private!();
}
/// [`Point`] is a trait that extends the [`RawPoint`] trait to provide additional
/// functionality for points in a hypergraph, such as accessing the index and raw index.
pub trait Point: RawPoint {
    /// returns the index of the point as a [`VertexId`].
    fn index(&self) -> &VertexId<Self::Key>;
    /// returns the raw index of the point as a reference to the underlying key type.
    fn raw_index(&self) -> &Self::Key;
}

pub trait WeightedPoint<T>: Point {
    /// returns an immutable reference to the weight of the point
    fn weight(&self) -> &Weight<T>;
    /// returns a mutable reference to the weight of the point
    fn weight_mut(&mut self) -> &mut Weight<T>;
    /// update the weight and return a mutable reference to the current instance
    fn set_weight(&mut self, weight: Weight<T>) -> &mut Self {
        self.weight_mut().set(weight.value());
        self
    }

    fn replace_weight(&mut self, weight: Weight<T>) -> T {
        self.weight_mut().replace(weight.value())
    }
}

/// Extends the base [HyperNode] trait with the [`Eq`] and [`Hash`](core::hash::Hash) traits
/// for use with hash-related structures.
pub trait HashPoint: RawPoint + Eq + core::hash::Hash
where
    Self::Key: Eq + core::hash::Hash,
{
}

/*
 ************* Implementations *************
*/
use crate::cmp::HyperNode;

impl<T: RawIndex> RawPoint for VertexId<T> {
    type Key = T;

    seal!();
}

impl<T: RawIndex> Point for VertexId<T> {
    fn index(&self) -> &VertexId<Self::Key> {
        self
    }

    fn raw_index(&self) -> &Self::Key {
        self.get()
    }
}

impl<Idx> HashPoint for VertexId<Idx> where Idx: RawIndex + Eq + core::hash::Hash {}

impl<T, Idx: RawIndex> RawPoint for HyperNode<T, Idx> {
    type Key = Idx;

    seal!();
}

impl<T, Idx: RawIndex> Point for HyperNode<T, Idx> {
    fn index(&self) -> &VertexId<Self::Key> {
        self.index()
    }

    fn raw_index(&self) -> &Self::Key {
        self.index().get()
    }
}

impl<T, Idx> HashPoint for HyperNode<T, Idx>
where
    Idx: RawIndex + Eq + core::hash::Hash,
    T: Eq + core::hash::Hash,
{
}

impl<T, Idx> WeightedPoint<T> for HyperNode<T, Idx>
where
    Idx: RawIndex,
{
    fn weight(&self) -> &Weight<T> {
        self.weight()
    }

    fn weight_mut(&mut self) -> &mut Weight<T> {
        self.weight_mut()
    }
}

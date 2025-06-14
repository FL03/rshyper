/*
    appellation: surface <module>
    authors: @FL03
*/
use super::RawEdge;
use crate::Weight;
use crate::idx::EdgeId;

/// [`RawFacet`] extends the behaviour of a [`RawEdge`] to include a weight
pub trait RawFacet<T>: RawEdge {
    private!();
    /// Returns the index of the edge.
    fn weight(&self) -> &Weight<T>;
    /// returns a mutable reference to the edge data.
    fn weight_mut(&mut self) -> &mut Weight<T>;
    /// [`replace`](core::mem::replace) the weight of the edge with a new one, returning the
    /// previous value
    fn replace_weight(&mut self, weight: Weight<T>) -> Weight<T> {
        core::mem::replace(self.weight_mut(), weight)
    }
    /// overwrites the weight of the edge with a new one and returns a mutable reference to
    /// the edge.
    fn set_weight(&mut self, weight: T) -> &mut Self {
        self.weight_mut().set(weight);
        self
    }
    /// [`swap`](core::mem::swap) the weight of the edge with another weight
    fn swap_weight(&mut self, weight: &mut Weight<T>) {
        core::mem::swap(self.weight_mut(), weight)
    }
    /// [`take`](core::mem::take) the weight of the edge, replacing it with a default value
    fn take_weight(&mut self) -> Weight<T>
    where
        T: Default,
    {
        core::mem::take(self.weight_mut())
    }
}

/// [`HyperFacet`] extends the behaviour of a [`RawFacet`] to include various constructors and
/// other utility methods.
pub trait HyperFacet<T>: RawFacet<T> {
    /// creates a new facet with the given id and weight
    fn new(id: EdgeId<Self::Index>, store: Self::Store, weight: Weight<T>) -> Self;
}

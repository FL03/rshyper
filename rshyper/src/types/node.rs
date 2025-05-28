/*
    Appellation: node <module>
    Contrib: @FL03
*/
use crate::traits::Weighted;
use crate::types::VertexId;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize)
)]
pub struct Node<T = (), Idx = usize> {
    pub(crate) index: VertexId<Idx>,
    pub(crate) weight: T,
}

impl<T, Idx> Node<T, Idx> {
    pub fn new(index: VertexId<Idx>, weight: T) -> Self {
        Self { index, weight }
    }
    /// returns a pointer to the inner value
    pub const fn as_ptr(&self) -> *const T {
        core::ptr::from_ref(&self.weight)
    }

    /// returns a mutable pointer to the inner value
    pub fn as_mut_ptr(&mut self) -> *mut T {
        core::ptr::from_mut(&mut self.weight)
    }

    pub const fn index(&self) -> &VertexId<Idx> {
        &self.index
    }

    pub const fn weight(&self) -> &T {
        &self.weight
    }

    pub fn weight_mut(&mut self) -> &mut T {
        &mut self.weight
    }

    pub fn set_weight(&mut self, weight: T) {
        self.weight = weight;
    }

    pub fn replace_weight(&mut self, weight: T) -> T {
        core::mem::replace(&mut self.weight, weight)
    }

    pub fn swap_weight(&mut self, other: &mut Self) {
        core::mem::swap(&mut self.weight, &mut other.weight);
    }
}

impl<T, Idx> Weighted<Idx> for Node<T, Idx> {
    type Data = T;

    fn weight(&self) -> &Self::Data {
        &self.weight
    }
}

impl<T, Idx> core::convert::AsRef<T> for Node<T, Idx> {
    fn as_ref(&self) -> &T {
        &self.weight
    }
}

impl<T, Idx> core::convert::AsMut<T> for Node<T, Idx> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.weight
    }
}

impl<T, Idx> core::borrow::Borrow<VertexId<Idx>> for Node<T, Idx> {
    fn borrow(&self) -> &VertexId<Idx> {
        &self.index
    }
}

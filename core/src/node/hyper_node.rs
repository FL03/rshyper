/*
    Appellation: node <module>
    Contrib: @FL03
*/
use crate::{VertexId, Weight};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
pub struct Node<T = (), Idx = usize> {
    pub(crate) index: VertexId<Idx>,
    pub(crate) weight: Weight<T>,
}

impl<T, Idx> Node<T, Idx> {
    /// initialize a new instance with the given index and weight
    pub fn new(index: VertexId<Idx>, weight: T) -> Self {
        Self {
            index,
            weight: Weight(weight),
        }
    }    
    /// returns a new weighted node using the given value and the logical default for the index
    pub fn from_index(index: VertexId<Idx>) -> Self
    where
        T: Default,
    {
        Self {
            index,
            weight: Weight::default(),
        }
    }    
    /// creates a new node with the given index using the logical default for the weight.
    pub fn from_weight(weight: Weight<T>) -> Self
    where
        Idx: Default,
    {
        Self {
            index: VertexId::default(),
            weight,
        }
    }
    /// consumes the current instance to create another with the given index.
    pub fn with_index<I2>(self, index: VertexId<I2>) -> Node<T, I2> {
        Node {
            index,
            weight: self.weight,
        }
    }
    /// consumes the current instance to create another with the given weight.
    pub fn with_weight<U>(self, weight: Weight<U>) -> Node<U, Idx> {
        Node {
            index: self.index,
            weight,
        }
    }
    /// returns an immutable reference to the node index
    pub const fn index(&self) -> &VertexId<Idx> {
        &self.index
    }
    /// returns an immutable reference to the node weight
    pub const fn weight(&self) -> &Weight<T> {
        &self.weight
    }
    /// returns a mutable reference to the node weight
    pub const fn weight_mut(&mut self) -> &mut Weight<T> {
        &mut self.weight
    }
    /// update the weight and return a mutable reference to the current instance.
    pub fn set_weight(&mut self, weight: T) -> &mut Self {
        self.weight_mut().set(weight);
        self
    }
    /// [`replace`](core::mem::replace) the weight of the current instance with the given weight,
    /// returning the previous weight.
    pub fn replace_weight(&mut self, weight: T) -> T {
        self.weight_mut().replace(weight)
    }
    /// [`swap`](core::mem::swap) the weight of the current instance with the weight of
    /// another instance.
    pub fn swap_weight(&mut self, other: &mut Self) {
        self.weight_mut().swap(other.weight_mut());
    }
    /// consumes the current instance and applies the given function onto the weight,
    /// returning a new instance with the same index and the resulting weight.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Node<U, Idx> {
        Node {
            index: self.index,
            weight: self.weight.map(f),
        }
    }
}

impl<T, Idx> AsRef<Weight<T>> for Node<T, Idx> {
    fn as_ref(&self) -> &Weight<T> {
        &self.weight
    }
}

impl<T, Idx> AsMut<Weight<T>> for Node<T, Idx> {
    fn as_mut(&mut self) -> &mut Weight<T> {
        &mut self.weight
    }
}

impl<T, Idx> core::borrow::Borrow<VertexId<Idx>> for Node<T, Idx> {
    fn borrow(&self) -> &VertexId<Idx> {
        &self.index
    }
}

impl<T, Idx> core::fmt::Display for Node<T, Idx>
where
    Idx: core::fmt::Display,
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ index: {}, weight: {} }}",
            self.index(),
            self.weight()
        )
    }
}

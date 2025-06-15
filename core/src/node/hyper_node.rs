/*
    Appellation: node <module>
    Contrib: @FL03
*/
use crate::Weight;
use crate::idx::{RawIndex, VertexId};

/// The [`Node`] implementation generically associates a [`VertexId`] with a [`Weight`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct Node<T, Idx>
where
    Idx: RawIndex,
{
    pub(crate) id: VertexId<Idx>,
    pub(crate) weight: Weight<T>,
}

impl<T, Idx> Node<T, Idx>
where
    Idx: RawIndex,
{
    /// initialize a new instance with the given index and weight
    pub const fn new(id: VertexId<Idx>, weight: T) -> Self {
        Self {
            id,
            weight: Weight(weight),
        }
    }
    /// returns a new weighted node using the given value and the logical default for the index
    pub fn from_id(index: VertexId<Idx>) -> Self
    where
        T: Default,
    {
        Self::new(index, Default::default())
    }
    /// creates a new node with the given index using the logical default for the weight.
    pub fn from_weight(Weight(weight): Weight<T>) -> Self
    where
        Idx: Default,
    {
        Self::new(Default::default(), weight)
    }
    /// consumes the current instance to create another with the given index.
    pub fn with_id<I2: RawIndex>(self, index: VertexId<I2>) -> Node<T, I2> {
        Node {
            id: index,
            weight: self.weight,
        }
    }
    /// consumes the current instance to create another with the given weight.
    pub fn with_weight<U>(self, weight: Weight<U>) -> Node<U, Idx> {
        Node {
            id: self.id,
            weight,
        }
    }
    /// converts the node into a 2-tuple consisting of the node index and the weight:
    /// 
    /// - `0`: a reference to the node index
    /// - `1`: a reference to the node weight
    pub const fn as_tuple(&self) -> (&VertexId<Idx>, &Weight<T>) {
        (self.id(), self.weight())
    }
    /// consumes the node to convert it into a 2-tuple consisting of the node index and the 
    /// weight
    pub fn into_tuple(self) -> (VertexId<Idx>, Weight<T>) {
        (self.id, self.weight)
    }
    /// returns the node as a tuple with a mutable reference to the weight such that:
    /// 
    /// -  `0`: a reference to the node index
    /// -  `1`: a mutable reference to the node weight
    /// 
    /// this method is useful for converting the node into a standard item produced by mutable 
    /// key-value iterators where `Item = (&'a K, &'a mut V)`
    pub fn as_tuple_mut(&mut self) -> (&VertexId<Idx>, &mut Weight<T>) {
        (&self.id, &mut self.weight)
    }
    /// returns an immutable reference to the node index
    pub const fn id(&self) -> &VertexId<Idx> {
        &self.id
    }
    /// returns an immutable reference to the node weight
    pub const fn weight(&self) -> &Weight<T> {
        &self.weight
    }
    /// returns a mutable reference to the node weight
    pub const fn weight_mut(&mut self) -> &mut Weight<T> {
        &mut self.weight
    }
    /// update the node id and return a mutable reference to the current instance.
    pub fn set_id(&mut self, id: VertexId<Idx>) -> &mut Self {
        self.id = id;
        self
    }
    /// update the weight and return a mutable reference to the current instance.
    pub fn set_weight(&mut self, weight: T) -> &mut Self {
        self.weight_mut().set(weight);
        self
    }
    /// [`replace`](core::mem::replace) the weight of the current instance with the given weight,
    /// returning the previous weight.
    pub const fn replace_weight(&mut self, weight: T) -> Weight<T> {
        let prev = self.weight_mut().replace(weight);
        Weight(prev)
    }
    /// [`swap`](core::mem::swap) the weight of the current instance with the weight of
    /// another instance.
    pub fn swap_weight(&mut self, other: &mut Self) {
        self.weight_mut().swap(other.weight_mut());
    }
    /// consumes the current instance and applies the given function onto the weight,
    /// returning a new instance with the same index and the resulting weight.
    pub fn map<U, F>(self, f: F) -> Node<U, Idx>
    where
        F: FnOnce(T) -> U,
    {
        Node {
            id: self.id,
            weight: self.weight.map(f),
        }
    }
    /// consumes the current instance and applies the given function onto the weight,
    pub fn map_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut T),
    {
        self.weight_mut().map_mut(f);
        self
    }
}

#[doc(hidden)]
#[allow(deprecated)]
impl<T, Idx> Node<T, Idx>
where
    Idx: RawIndex,
{
    #[deprecated(
        note = "use `from_id` instead; the constructor will be removed in the next major release",
        since = "0.1.2"
    )]
    pub fn from_index(index: VertexId<Idx>) -> Self
    where
        T: Default,
    {
        Self::new(index, Default::default())
    }
    #[deprecated(
        note = "use `id` instead; the accessor will be removed in the next major release",
        since = "0.1.2"
    )]
    pub const fn index(&self) -> &VertexId<Idx> {
        self.id()
    }
    #[deprecated(
        note = "use `with_id` instead; the setter will be removed in the next major release",
        since = "0.1.2"
    )]
    pub fn with_index<I2: RawIndex>(self, index: VertexId<I2>) -> Node<T, I2> {
        Node {
            id: index,
            weight: self.weight,
        }
    }
}

impl<T, Idx> Default for Node<T, Idx>
where
    Idx: RawIndex + Default,
    T: Default,
{
    fn default() -> Self {
        Self {
            id: VertexId::default(),
            weight: Weight::default(),
        }
    }
}

impl<T, Idx> core::fmt::Display for Node<T, Idx>
where
    Idx: RawIndex + core::fmt::Display,
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ index: {}, weight: {} }}", self.id(), self.weight())
    }
}

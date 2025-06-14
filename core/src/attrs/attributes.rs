/*
    appellation: attrs <module>
    authors: @FL03
*/
use crate::idx::RawIndex;
use crate::{Directed, GraphType, Mode, Undirected};
use core::marker::PhantomData;

/// [`Attributes`] is a generic implementation of the [`GraphAttributes`](super::GraphAttributes) trait enabling the
/// definition of hypergraphs with different index types and graph kinds (directed or
/// undirected).
#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
pub struct Attributes<I = usize, K = Undirected>
where
    I: RawIndex,
    K: GraphType,
{
    /// the inner type of index used by the graph
    pub(crate) index: PhantomData<I>,
    /// the kind of graph, either directed or undirected
    pub(crate) kind: PhantomData<K>,
}

impl<I, K> Attributes<I, K>
where
    I: RawIndex,
    K: GraphType,
{
    /// returns a new instance of [`Attributes`] initialized with the given index and kind.
    pub const fn new() -> Self {
        Attributes {
            index: PhantomData::<I>,
            kind: PhantomData::<K>,
        }
    }
    /// consumes the current instance to create another with the given kind
    pub fn with_kind<K2>(self) -> Attributes<I, K2>
    where
        K2: GraphType,
    {
        Attributes {
            index: self.index,
            kind: PhantomData::<K2>,
        }
    }
    /// consumes the current instance to create another with the given index type
    pub fn with_index<I2>(self) -> Attributes<I2, K>
    where
        I2: RawIndex,
    {
        Attributes {
            index: PhantomData::<I2>,
            kind: self.kind,
        }
    }
    /// returns true if the current kind `K` is the same as the given kind `K2`
    pub fn is_kind<K2: 'static>(&self) -> bool {
        use core::any::TypeId;
        TypeId::of::<K2>() == TypeId::of::<K>()
    }

    /// returns true if the current index type `I` is the same as the given index type `I2`
    pub fn is_index<I2: 'static>(&self) -> bool {
        use core::any::TypeId;
        TypeId::of::<I2>() == TypeId::of::<I>()
    }
    /// returns true if the current kind is [`Directed`]
    pub fn is_directed(&self) -> bool {
        self.is_kind::<Directed>()
    }
    /// returns true if the current kind is [`Undirected`]
    pub fn is_undirected(&self) -> bool {
        self.is_kind::<Undirected>()
    }
    /// returns the _kind_ of the graph as an enum variant of [`Mode`]
    pub fn mode(&self) -> Mode {
        if self.is_directed() {
            Mode::Directed
        } else if self.is_undirected() {
            Mode::Undirected
        } else {
            panic!("Unknown graph type")
        }
    }
}

impl<I> Attributes<I, Directed>
where
    I: RawIndex,
{
    pub fn directed() -> Self {
        Attributes {
            index: PhantomData::<I>,
            kind: PhantomData::<Directed>,
        }
    }
}

impl<I> Attributes<I, Undirected>
where
    I: RawIndex,
{
    pub fn undirected() -> Self {
        Attributes {
            index: PhantomData::<I>,
            kind: PhantomData::<Undirected>,
        }
    }
}

impl<I, K> Clone for Attributes<I, K>
where
    I: RawIndex,
    K: GraphType,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<I, K> Copy for Attributes<I, K>
where
    I: RawIndex,
    K: GraphType,
{
}

impl<I, K> Default for Attributes<I, K>
where
    I: RawIndex,
    K: GraphType,
{
    fn default() -> Self {
        Attributes::new()
    }
}

unsafe impl<I, K> Send for Attributes<I, K>
where
    I: RawIndex,
    K: GraphType,
{
}

unsafe impl<I, K> Sync for Attributes<I, K>
where
    I: RawIndex,
    K: GraphType,
{
}

impl<I, K> core::fmt::Debug for Attributes<I, K>
where
    I: RawIndex,
    K: GraphType,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Attributes<{}, {}>",
            core::any::type_name::<I>(),
            core::any::type_name::<K>()
        )
    }
}

impl<I, K> core::fmt::Display for Attributes<I, K>
where
    I: RawIndex,
    K: GraphType,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Attributes<{}, {}>",
            core::any::type_name::<I>(),
            core::any::type_name::<K>()
        )
    }
}

/*
    appellation: hyper_edge <module>
    authors: @FL03
*/
use crate::index::{EdgeId, RawIndex};

/// [`HyperEdge`] is a type representing a hyperedge in a hypergraph.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct HyperEdge<S, Idx = usize>
where
    Idx: RawIndex,
{
    pub(crate) id: EdgeId<Idx>,
    pub(crate) nodes: S,
}

impl<S, Idx> HyperEdge<S, Idx>
where
    Idx: RawIndex,
{
    pub fn new(id: EdgeId<Idx>, nodes: S) -> Self {
        Self { id, nodes }
    }
    /// creates a new edge with the given id
    pub fn from_id(id: EdgeId<Idx>) -> Self
    where
        S: Default,
    {
        Self {
            id,
            nodes: Default::default(),
        }
    }
    /// creates a new edge with the given nodes
    pub fn from_nodes(nodes: S) -> Self
    where
        Idx: Default,
    {
        Self {
            id: EdgeId::default(),
            nodes,
        }
    }
    /// returns an immutable reference to the id
    pub const fn id(&self) -> &EdgeId<Idx> {
        &self.id
    }
    /// returns a mutable reference to the id
    pub const fn id_mut(&mut self) -> &mut EdgeId<Idx> {
        &mut self.id
    }
    /// returns an immutable reference to the nodes
    pub const fn nodes(&self) -> &S {
        &self.nodes
    }
    /// returns a mutable reference to the nodes
    pub const fn nodes_mut(&mut self) -> &mut S {
        &mut self.nodes
    }
    /// updates the id and returns a mutable reference to the instance
    pub fn set_id(&mut self, id: EdgeId<Idx>) -> &mut Self {
        self.id = id;
        self
    }
    /// updates the nodes and returns a mutable reference to the instance
    pub fn set_nodes(&mut self, nodes: S) -> &mut Self {
        self.nodes = nodes;
        self
    }
    /// consumes the current instance to create another with the given id.
    pub fn with_id<I2: RawIndex>(self, id: EdgeId<I2>) -> HyperEdge<S, I2> {
        HyperEdge {
            id,
            nodes: self.nodes,
        }
    }
    /// consumes the current instance to create another with the given nodes.
    pub fn with_nodes<S2>(self, nodes: S2) -> HyperEdge<S2, Idx> {
        HyperEdge { id: self.id, nodes }
    }
}

impl<S, Idx> AsRef<S> for HyperEdge<S, Idx>
where
    Idx: RawIndex,
{
    fn as_ref(&self) -> &S {
        self.nodes()
    }
}

impl<S, Idx> AsMut<S> for HyperEdge<S, Idx>
where
    Idx: RawIndex,
{
    fn as_mut(&mut self) -> &mut S {
        self.nodes_mut()
    }
}

impl<S, Idx> core::borrow::Borrow<EdgeId<Idx>> for HyperEdge<S, Idx>
where
    Idx: RawIndex,
{
    fn borrow(&self) -> &EdgeId<Idx> {
        self.id()
    }
}

impl<S, Idx> core::borrow::BorrowMut<EdgeId<Idx>> for HyperEdge<S, Idx>
where
    Idx: RawIndex,
{
    fn borrow_mut(&mut self) -> &mut EdgeId<Idx> {
        self.id_mut()
    }
}

impl<S, Idx> core::ops::Deref for HyperEdge<S, Idx>
where
    Idx: RawIndex,
{
    type Target = EdgeId<Idx>;

    fn deref(&self) -> &Self::Target {
        self.id()
    }
}

impl<S, Idx> core::ops::DerefMut for HyperEdge<S, Idx>
where
    Idx: RawIndex,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.id_mut()
    }
}

impl<S, Idx> core::fmt::Display for HyperEdge<S, Idx>
where
    Idx: RawIndex + core::fmt::Display,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(id: {}, nodes: {})", self.id, self.nodes,)
    }
}

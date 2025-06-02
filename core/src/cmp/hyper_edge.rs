/*
    appellation: hyper_edge <module>
    authors: @FL03
*/
use super::RawEdgeStore;
use crate::GraphKind;
use crate::index::{EdgeId, RawIndex};

/// [`HyperEdge`] is a type representing a hyperedge in a hypergraph.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct HyperEdge<S, K, Idx = usize>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    pub(crate) id: EdgeId<Idx>,
    pub(crate) points: S,
    pub(crate) _kind: core::marker::PhantomData<K>,
}

impl<S, K, Idx> HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    pub fn new(id: EdgeId<Idx>, points: S) -> Self {
        Self {
            id,
            points,
            _kind: core::marker::PhantomData::<K>,
        }
    }
    /// creates a new edge with the given id
    pub fn from_id(id: EdgeId<Idx>) -> Self
    where
        S: Default,
    {
        Self::new(id, Default::default())
    }
    /// creates a new edge with the given nodes
    pub fn from_points(nodes: S) -> Self
    where
        Idx: Default,
    {
        Self::new(Default::default(), nodes)
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
    pub const fn points(&self) -> &S {
        &self.points
    }
    /// returns a mutable reference to the nodes
    pub const fn points_mut(&mut self) -> &mut S {
        &mut self.points
    }
    /// updates the id and returns a mutable reference to the instance
    pub fn set_id(&mut self, id: EdgeId<Idx>) -> &mut Self {
        self.id = id;
        self
    }
    /// updates the nodes and returns a mutable reference to the instance
    pub fn set_points(&mut self, nodes: S) -> &mut Self {
        self.points = nodes;
        self
    }
    /// consumes the current instance to create another with the given id.
    pub fn with_id(self, id: EdgeId<Idx>) -> Self {
        Self { id, ..self }
    }
    /// consumes the current instance to create another with the given nodes.
    pub fn with_points<S2: RawEdgeStore<Idx>>(self, nodes: S2) -> HyperEdge<S2, K, Idx> {
        HyperEdge {
            id: self.id,
            points: nodes,
            _kind: self._kind,
        }
    }
}

impl<S, Idx> HyperEdge<S, crate::Directed, Idx>
where
    Idx: RawIndex,
    S: RawEdgeStore<Idx>,
{
    /// creates a new directed hyperedge with the given id and nodes
    pub fn directed(id: EdgeId<Idx>, nodes: S) -> Self {
        Self::new(id, nodes)
    }
}

impl<S, Idx> HyperEdge<S, crate::Undirected, Idx>
where
    Idx: RawIndex,
    S: RawEdgeStore<Idx>,
{
    /// creates a new undirected hyperedge with the given id and nodes
    pub fn undirected(id: EdgeId<Idx>, nodes: S) -> Self {
        Self::new(id, nodes)
    }
}

impl<S, K, Idx> Default for HyperEdge<S, K, Idx>
where
    Idx: RawIndex + Default,
    K: GraphKind,
    S: RawEdgeStore<Idx> + Default,
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<S, K, Idx> AsRef<S> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    fn as_ref(&self) -> &S {
        self.points()
    }
}

impl<S, K, Idx> AsMut<S> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    fn as_mut(&mut self) -> &mut S {
        self.points_mut()
    }
}

impl<S, K, Idx> core::borrow::Borrow<EdgeId<Idx>> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    fn borrow(&self) -> &EdgeId<Idx> {
        self.id()
    }
}

impl<S, K, Idx> core::borrow::BorrowMut<EdgeId<Idx>> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    fn borrow_mut(&mut self) -> &mut EdgeId<Idx> {
        self.id_mut()
    }
}

impl<S, K, Idx> core::ops::Deref for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    type Target = EdgeId<Idx>;

    fn deref(&self) -> &Self::Target {
        self.id()
    }
}

impl<S, K, Idx> core::ops::DerefMut for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.id_mut()
    }
}

impl<S, K, Idx> core::fmt::Display for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx> + core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(id: {}, nodes: {})", self.id, self.points,)
    }
}

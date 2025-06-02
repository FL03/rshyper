/*
    appellation: hyper_edge <module>
    authors: @FL03
*/

mod impl_edge;

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
    /// returns true if the edge contains the given vertex index
    pub fn contains_vertex(&self, index: &crate::VertexId<Idx>) -> bool
    where
        Idx: PartialEq,
        for<'a> &'a S: IntoIterator<Item = &'a crate::VertexId<Idx>>,
    {
        self.points().into_iter().any(|v| v == index)
    }
    /// returns the number of vertices in the edge
    pub fn len(&self) -> usize
    where
        S: crate::cmp::RawEdgeStore<Idx>,
    {
        self.points().len()
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

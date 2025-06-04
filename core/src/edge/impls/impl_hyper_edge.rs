/*
    appellation: impl_edge <module>
    authors: @FL03
*/
use crate::edge::{HyperEdge, RawStore};
use crate::index::{EdgeId, RawIndex, VertexId};
use crate::{Directed, GraphKind, Undirected};

impl<S, K, Idx> HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
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
    pub fn with_points<S2: RawStore<Idx>>(self, nodes: S2) -> HyperEdge<S2, K, Idx> {
        HyperEdge {
            id: self.id,
            points: nodes,
            _kind: self._kind,
        }
    }
    /// returns true if the edge contains the given vertex index
    pub fn contains_vertex<Q>(&self, index: &Q) -> bool
    where
        VertexId<Idx>: core::borrow::Borrow<Q>,
        Q: PartialEq,
        Idx: PartialEq,
        for<'a> &'a S: IntoIterator<Item = &'a VertexId<Idx>>,
    {
        use core::borrow::Borrow;
        self.points().into_iter().any(|v| v.borrow() == index)
    }
    /// returns the number of vertices in the edge
    pub fn len(&self) -> usize
    where
        S: RawStore<Idx>,
    {
        self.points().len()
    }
}

impl<S, Idx> HyperEdge<S, Directed, Idx>
where
    Idx: RawIndex,
    S: RawStore<Idx>,
{
    /// creates a new directed hyperedge with the given id and nodes
    pub fn directed(id: EdgeId<Idx>, nodes: S) -> Self {
        Self::new(id, nodes)
    }
}

impl<S, Idx> HyperEdge<S, Undirected, Idx>
where
    Idx: RawIndex,
    S: RawStore<Idx>,
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
    S: RawStore<Idx> + Default,
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<S, K, Idx> core::fmt::Display for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx> + core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(id: {}, nodes: {})", self.id, self.points,)
    }
}

impl<S, K, Idx> FromIterator<VertexId<Idx>> for HyperEdge<S, K, Idx>
where
    Idx: Default + RawIndex,
    K: GraphKind,
    S: RawStore<Idx> + FromIterator<VertexId<Idx>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = VertexId<Idx>>,
    {
        let store = S::from_iter(iter);
        Self::from_points(store)
    }
}

impl<S, K, Idx> From<EdgeId<Idx>> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: Default + RawStore<Idx>,
{
    fn from(from: EdgeId<Idx>) -> Self {
        Self::from_id(from)
    }
}

impl<S, K, Idx> From<HyperEdge<S, K, Idx>> for EdgeId<Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn from(from: HyperEdge<S, K, Idx>) -> Self {
        from.id
    }
}

impl<S, K, Idx> AsRef<S> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn as_ref(&self) -> &S {
        self.points()
    }
}

impl<S, K, Idx> AsMut<S> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn as_mut(&mut self) -> &mut S {
        self.points_mut()
    }
}

impl<S, K, Idx> core::borrow::Borrow<EdgeId<Idx>> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn borrow(&self) -> &EdgeId<Idx> {
        self.id()
    }
}

impl<S, K, Idx> core::borrow::BorrowMut<EdgeId<Idx>> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn borrow_mut(&mut self) -> &mut EdgeId<Idx> {
        self.id_mut()
    }
}

impl<S, K, Idx> core::ops::Deref for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
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
    S: RawStore<Idx>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.id_mut()
    }
}

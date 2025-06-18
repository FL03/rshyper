/*
    appellation: hyper_edge <module>
    authors: @FL03
*/
use super::{Layout, RawLayout};
use crate::idx::{EdgeId, RawIndex, VertexId};
use crate::{Domain, GraphType};

/// [`Link`] is the base type for hyperedges in a graph. These edges are generic over the
/// edge store type `S`, the graph kind `K`, and the index type `Idx`. This allows for
/// flexibility in how edges store their vertices and how they are identified within the graph.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct Link<S, K, Ix = usize>
where
Ix: RawIndex,
    K: GraphType,
    S: Domain<Ix>,
{
    pub(crate) id: EdgeId<Ix>,
    pub(crate) domain: S,
    pub(crate) _kind: core::marker::PhantomData<K>,
}

impl<S, K, Ix> Link<S, K, Ix>
where
    Ix: RawIndex,
    K: GraphType,
    S: Domain<Ix>,
{
    pub fn new(id: EdgeId<Ix>, domain: S) -> Self {
        Self {
            id,
            domain,
            _kind: core::marker::PhantomData::<K>,
        }
    }
    /// creates a new edge with the given id
    pub fn from_id(id: EdgeId<Ix>) -> Self
    where
        S: Default,
    {
        Self::new(id, Default::default())
    }
    /// creates a new edge with the given nodes
    pub fn from_domain(nodes: S) -> Self
    where
        Ix: Default,
    {
        Self::new(Default::default(), nodes)
    }
    /// returns an immutable reference to the id
    pub const fn id(&self) -> &EdgeId<Ix> {
        &self.id
    }
    /// returns a mutable reference to the id
    pub const fn id_mut(&mut self) -> &mut EdgeId<Ix> {
        &mut self.id
    }
    /// returns an immutable reference to the nodes
    pub const fn domain(&self) -> &S {
        &self.domain
    }
    /// returns a mutable reference to the nodes
    pub const fn domain_mut(&mut self) -> &mut S {
        &mut self.domain
    }
    /// updates the nodes and returns a mutable reference to the instance
    pub fn set_domain(&mut self, nodes: S) -> &mut Self {
        self.domain = nodes;
        self
    }
    /// consumes the current instance to create another with the given id.
    pub fn with_id(self, id: EdgeId<Ix>) -> Self {
        Self { id, ..self }
    }
    /// consumes the current instance to create another with the given nodes.
    pub fn with_domain<S2: Domain<Ix>>(self, nodes: S2) -> Link<S2, K, Ix> {
        Link {
            id: self.id,
            domain: nodes,
            _kind: self._kind,
        }
    }
    /// returns true if the edge contains the given vertex index
    pub fn contains<Q>(&self, index: &Q) -> bool
    where
        Ix: PartialEq,
        Q: ?Sized + PartialEq,
        VertexId<Ix>: core::borrow::Borrow<Q>,
        for<'a> &'a S: IntoIterator<Item = &'a VertexId<Ix>>,
    {
        use core::borrow::Borrow;
        self.domain().into_iter().any(|v| v.borrow() == index)
    }
    /// returns true if the edge contains all the given vertex indices
    pub fn contains_all<Q, I>(&self, indices: I) -> bool
    where
        Ix: PartialEq,
        I: IntoIterator<Item = Q>,
        Q: PartialEq,
        VertexId<Ix>: core::borrow::Borrow<Q>,
        for<'a> &'a S: IntoIterator<Item = &'a VertexId<Ix>>,
    {
        indices.into_iter().all(|index| self.contains(&index))
    }
    /// returns the number of vertices in the edge
    pub fn len(&self) -> usize
    where
        S: Domain<Ix>,
    {
        self.domain().len()
    }
    /// returns true if the edge has no vertices
    pub fn is_empty(&self) -> bool
    where
        S: Domain<Ix>,
    {
        self.domain().is_empty()
    }
}
/// private implementations for the [`Link`] struct
#[doc(hidden)]
#[allow(dead_code)]
impl<S, K, Idx> Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    #[doc(hidden)]
    /// updates the id and returns a mutable reference to the instance
    pub(crate) fn set_id(&mut self, id: EdgeId<Idx>) -> &mut Self {
        self.id = id;
        self
    }
}

#[doc(hidden)]
#[allow(deprecated)]
impl<S, K, Idx> Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    #[deprecated(
        note = "Use `Edge::from_domain` instead. This method will be removed in a future version",
        since = "0.1.2"
    )]
    pub fn from_points(nodes: S) -> Self
    where
        Idx: Default,
    {
        Self::from_domain(nodes)
    }
    #[deprecated(
        note = "Use `Edge::domain` instead. This method will be removed in a future version",
        since = "0.1.2"
    )]
    /// returns an immutable reference to the nodes
    pub const fn points(&self) -> &S {
        self.domain()
    }
    #[deprecated(
        note = "Use `Edge::domain_mut` instead. This method will be removed in a future version",
        since = "0.1.2"
    )]
    /// returns a mutable reference to the nodes
    pub const fn points_mut(&mut self) -> &mut S {
        self.domain_mut()
    }
    #[deprecated(
        note = "Use `Edge::set_domain` instead. This method will be removed in a future version",
        since = "0.1.2"
    )]
    pub fn set_points(&mut self, nodes: S) -> &mut Self {
        self.set_domain(nodes)
    }
    #[deprecated(
        note = "Use `Edge::with_domain` instead. This method will be removed in a future version",
        since = "0.1.2"
    )]
    pub fn with_points<S2: Domain<Idx>>(self, nodes: S2) -> Link<S2, K, Idx> {
        self.with_domain(nodes)
    }
}

impl<S, K, Idx> RawLayout for Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    type Kind = K;
    type Index = Idx;
    type Store = S;

    seal!();

    fn index(&self) -> &EdgeId<Idx> {
        self.id()
    }

    fn domain(&self) -> &S {
        self.domain()
    }

    fn domain_mut(&mut self) -> &mut S {
        self.domain_mut()
    }
}

impl<S, K, Idx> Layout for Link<S, K, Idx>
where
    S: Domain<Idx>,
    Idx: RawIndex,
    K: GraphType,
{
    fn new(id: EdgeId<Idx>, vertices: S) -> Self {
        Self::new(id, vertices)
    }
}

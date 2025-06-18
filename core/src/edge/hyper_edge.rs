/*
    Appellation: edge <module>
    Contrib: @FL03
*/
use super::RawSurface;
use crate::idx::{EdgeId, RawIndex, VertexId};
use crate::rel::{Link, RawLayout};
use crate::{Domain, GraphType, Weight};

/// The [`Edge`] implementation essentially wraps the [`Link`] type with a [`Weight`]
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct Edge<T, S, K, Idx = usize>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    pub(crate) link: Link<S, K, Idx>,
    pub(crate) weight: Weight<T>,
}

impl<T, S, K, Idx> Edge<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    /// creates a new instance from the given edge and weight
    pub const fn new(edge: Link<S, K, Idx>, weight: Weight<T>) -> Self {
        Self { link: edge, weight }
    }
    /// create a new instance of the [`Edge`] from the given id, nodes, and weight
    pub fn from_parts(id: EdgeId<Idx>, nodes: S, weight: Weight<T>) -> Self {
        let edge = Link::new(id, nodes);
        Self::new(edge, weight)
    }
    /// creates a new edge with the given nodes
    pub fn from_domain(nodes: S) -> Self
    where
        Idx: Default,
        T: Default,
    {
        let rel = Link::from_domain(nodes);
        Self::new(rel, Default::default())
    }
    /// creates a new edge with the given id
    pub fn from_id(id: EdgeId<Idx>) -> Self
    where
        S: Default,
        T: Default,
    {
        Self {
            link: Link::from_id(id),
            weight: Weight::default(),
        }
    }
    /// creates a new edge with the given id
    pub fn from_link(edge: Link<S, K, Idx>) -> Self
    where
        T: Default,
    {
        Self::new(edge, Default::default())
    }
    /// creates a new instance from the given value
    pub fn from_weight(weight: Weight<T>) -> Self
    where
        Idx: Default,
        S: Default,
    {
        Self {
            link: Default::default(),
            weight,
        }
    }
    /// returns an immutable reference to the edge
    pub const fn link(&self) -> &Link<S, K, Idx> {
        &self.link
    }
    /// returns a mutable reference to the edge
    pub const fn link_mut(&mut self) -> &mut Link<S, K, Idx> {
        &mut self.link
    }
    /// returns an immutable reference to the weight
    pub const fn weight(&self) -> &Weight<T> {
        &self.weight
    }
    /// returns a mutable reference to the weight
    pub const fn weight_mut(&mut self) -> &mut Weight<T> {
        &mut self.weight
    }
    /// returns an immutable reference to the id
    pub const fn id(&self) -> &EdgeId<Idx> {
        self.link().id()
    }
    /// returns a mutable reference to the id
    pub const fn id_mut(&mut self) -> &mut EdgeId<Idx> {
        self.link_mut().id_mut()
    }
    /// returns an immutable reference to the nodes
    pub const fn domain(&self) -> &S {
        self.link().domain()
    }
    /// returns a mutable reference to the nodes
    pub const fn domain_mut(&mut self) -> &mut S {
        self.link_mut().domain_mut()
    }
    /// updates the nodes and returns a mutable reference to the instance
    pub fn set_domain(&mut self, nodes: S) -> &mut Self {
        self.link_mut().set_domain(nodes);
        self
    }
    /// updates the weight and returns a mutable reference to the instance
    pub fn set_weight(&mut self, weight: Weight<T>) -> &mut Self {
        self.weight = weight;
        self
    }
    /// consumes the current instance to create another with the given id.
    pub fn with_id(self, id: EdgeId<Idx>) -> Self {
        Edge {
            link: self.link.with_id(id),
            weight: self.weight,
        }
    }
    /// consumes the current instance to create another with the given nodes.
    pub fn with_domain<S2: Domain<Idx>>(self, nodes: S2) -> Edge<T, S2, K, Idx> {
        Edge {
            link: self.link.with_domain(nodes),
            weight: self.weight,
        }
    }
    /// consumes the current instance to create another with the given weight.
    pub fn with_weight<U>(self, weight: Weight<U>) -> Edge<U, S, K, Idx> {
        Edge {
            link: self.link,
            weight,
        }
    }
    /// returns true if the edge contains the given vertex
    pub fn contains<Q>(&self, index: &Q) -> bool
    where
        VertexId<Idx>: core::borrow::Borrow<Q>,
        Q: ?Sized + PartialEq,
        Idx: PartialEq,
        for<'a> &'a S: IntoIterator<Item = &'a VertexId<Idx>>,
    {
        self.link().contains(index)
    }
    /// returns true if the edge is empty
    pub fn is_empty(&self) -> bool {
        self.link().is_empty()
    }
    /// returns the number of nodes in the edge
    pub fn len(&self) -> usize {
        self.link().len()
    }
}

#[allow(deprecated)]
#[doc(hidden)]
impl<T, S, K, Idx> Edge<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    #[deprecated(
        note = "Use `domain` instead. This method will be removed in the next major version.",
        since = "0.1.2"
    )]
    pub const fn nodes(&self) -> &S {
        self.link().domain()
    }
    #[deprecated(
        note = "Use `domain_mut` instead. This method will be removed in the next major version.",
        since = "0.1.2"
    )]
    pub const fn nodes_mut(&mut self) -> &mut S {
        self.link_mut().domain_mut()
    }
}

impl<T, S, Idx, K> RawLayout for Edge<T, S, K, Idx>
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
        self.link().id()
    }

    fn domain(&self) -> &S {
        self.link().domain()
    }

    fn domain_mut(&mut self) -> &mut S {
        self.link_mut().domain_mut()
    }
}

impl<T, S, Idx, K> RawSurface<T> for Edge<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    seal!();

    fn weight(&self) -> &Weight<T> {
        self.as_ref()
    }

    fn weight_mut(&mut self) -> &mut Weight<T> {
        self.as_mut()
    }
}

/*
    appellation: hyper_facet <module>
    authors: @FL03
*/
use super::HyperEdge;
use crate::Weight;
use crate::index::{EdgeId, RawIndex};

/// The [`HyperFacet`] implementation associates some weight with a hyperedge.
/// Typically, the term **facet** is used to denote the surface of a particular polytope,
/// however, here it is used to aptly define a _**weighted**_ hyperedge.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct HyperFacet<T, S, Idx = usize>
where
    Idx: RawIndex,
{
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(crate) edge: HyperEdge<S, Idx>,
    pub(crate) weight: Weight<T>,
}

impl<T, S, Idx> HyperFacet<T, S, Idx>
where
    Idx: RawIndex,
{
    /// create a new instance of the [`HyperFacet`] from the given id, nodes, and weight
    pub fn new(id: EdgeId<Idx>, nodes: S, weight: T) -> Self {
        let edge = HyperEdge::new(id, nodes);
        Self {
            edge,
            weight: Weight(weight),
        }
    }
    /// creates a new edge with the given id
    pub fn from_edge(edge: HyperEdge<S, Idx>) -> Self
    where
        S: Default,
        T: Default,
    {
        Self::from_edge_with_weight(edge, Default::default())
    }
    /// creates a new instance from the given edge and weight
    pub fn from_edge_with_weight(edge: HyperEdge<S, Idx>, weight: Weight<T>) -> Self
    where
        S: Default,
    {
        Self { edge, weight }
    }
    /// creates a new edge with the given id
    pub fn from_id(id: EdgeId<Idx>) -> Self
    where
        S: Default,
        T: Default,
    {
        Self {
            edge: HyperEdge::from_id(id),
            weight: Weight::default(),
        }
    }
    /// creates a new edge with the given nodes
    pub fn from_nodes(nodes: S) -> Self
    where
        Idx: Default,
        T: Default,
    {
        Self {
            edge: HyperEdge::from_nodes(nodes),
            weight: Weight::default(),
        }
    }
    /// creates a new instance from the given value
    pub fn from_weight(weight: Weight<T>) -> Self
    where
        Idx: Default,
        S: Default,
    {
        Self {
            edge: HyperEdge::default(),
            weight,
        }
    }
    /// returns an immutable reference to the edge
    pub const fn edge(&self) -> &HyperEdge<S, Idx> {
        &self.edge
    }
    /// returns a mutable reference to the edge
    pub const fn edge_mut(&mut self) -> &mut HyperEdge<S, Idx> {
        &mut self.edge
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
        self.edge().id()
    }
    /// returns a mutable reference to the id
    pub const fn id_mut(&mut self) -> &mut EdgeId<Idx> {
        self.edge_mut().id_mut()
    }
    /// returns an immutable reference to the nodes
    pub const fn nodes(&self) -> &S {
        self.edge().nodes()
    }
    /// returns a mutable reference to the nodes
    pub const fn nodes_mut(&mut self) -> &mut S {
        self.edge_mut().nodes_mut()
    }
    /// updates the id and returns a mutable reference to the instance
    pub fn set_id(&mut self, id: EdgeId<Idx>) -> &mut Self {
        self.edge_mut().set_id(id);
        self
    }
    /// updates the nodes and returns a mutable reference to the instance
    pub fn set_nodes(&mut self, nodes: S) -> &mut Self {
        self.edge_mut().set_nodes(nodes);
        self
    }
    /// updates the weight and returns a mutable reference to the instance
    pub fn set_weight(&mut self, weight: Weight<T>) -> &mut Self {
        self.weight = weight;
        self
    }
    /// consumes the current instance to create another with the given id.
    pub fn with_id<I2: RawIndex>(self, id: EdgeId<I2>) -> HyperFacet<T, S, I2> {
        HyperFacet {
            edge: self.edge.with_id(id),
            weight: self.weight,
        }
    }
    /// consumes the current instance to create another with the given nodes.
    pub fn with_nodes<S2>(self, nodes: S2) -> HyperFacet<T, S2, Idx> {
        HyperFacet {
            edge: self.edge.with_nodes(nodes),
            weight: self.weight,
        }
    }
    /// consumes the current instance to create another with the given weight.
    pub fn with_weight<U>(self, weight: Weight<U>) -> HyperFacet<U, S, Idx> {
        HyperFacet {
            edge: self.edge,
            weight,
        }
    }
}

impl<T, S, Idx> AsRef<Weight<T>> for HyperFacet<T, S, Idx>
where
    Idx: RawIndex,
{
    fn as_ref(&self) -> &Weight<T> {
        &self.weight
    }
}

impl<T, S, Idx> AsMut<Weight<T>> for HyperFacet<T, S, Idx>
where
    Idx: RawIndex,
{
    fn as_mut(&mut self) -> &mut Weight<T> {
        &mut self.weight
    }
}

impl<T, S, Idx> core::borrow::Borrow<EdgeId<Idx>> for HyperFacet<T, S, Idx>
where
    Idx: RawIndex,
{
    fn borrow(&self) -> &EdgeId<Idx> {
        self.id()
    }
}

impl<T, S, Idx> core::borrow::BorrowMut<EdgeId<Idx>> for HyperFacet<T, S, Idx>
where
    Idx: RawIndex,
{
    fn borrow_mut(&mut self) -> &mut EdgeId<Idx> {
        self.id_mut()
    }
}

impl<T, S, Idx> core::ops::Deref for HyperFacet<T, S, Idx>
where
    Idx: RawIndex,
{
    type Target = HyperEdge<S, Idx>;

    fn deref(&self) -> &Self::Target {
        self.edge()
    }
}

impl<T, S, Idx> core::ops::DerefMut for HyperFacet<T, S, Idx>
where
    Idx: RawIndex,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.edge_mut()
    }
}

impl<T, S, Idx> core::fmt::Display for HyperFacet<T, S, Idx>
where
    Idx: RawIndex + core::fmt::Display,
    T: core::fmt::Display,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ edge: {}, weight: {} }}", self.edge, self.weight)
    }
}

/*
    Appellation: node <module>
    Contrib: @FL03
*/
use super::{HyperEdge, RawEdge, RawFacet, RawStore};
use crate::index::{EdgeId, RawIndex, VertexId};
use crate::{GraphKind, Weight};

/// The [`HyperFacet`] implementation associates some weight with a hyperedge.
/// Typically, the term **facet** is used to denote the surface of a particular polytope,
/// however, here it is used to aptly define a _**weighted**_ hyperedge.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct HyperFacet<T, S, K, Idx = usize>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(crate) edge: HyperEdge<S, K, Idx>,
    pub(crate) weight: Weight<T>,
}

impl<T, S, K, Idx> HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    /// create a new instance of the [`HyperFacet`] from the given id, nodes, and weight
    pub fn new(id: EdgeId<Idx>, nodes: S, weight: Weight<T>) -> Self {
        let edge = HyperEdge::new(id, nodes);
        Self { edge, weight }
    }
    /// creates a new edge with the given id
    pub fn from_edge(edge: HyperEdge<S, K, Idx>) -> Self
    where
        T: Default,
    {
        Self::from_edge_with_weight(edge, Default::default())
    }
    /// creates a new instance from the given edge and weight
    pub fn from_edge_with_weight(edge: HyperEdge<S, K, Idx>, weight: Weight<T>) -> Self {
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
    pub fn from_points(nodes: S) -> Self
    where
        Idx: Default,
        T: Default,
    {
        Self {
            edge: HyperEdge::from_points(nodes),
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
    pub const fn edge(&self) -> &HyperEdge<S, K, Idx> {
        &self.edge
    }
    /// returns a mutable reference to the edge
    pub const fn edge_mut(&mut self) -> &mut HyperEdge<S, K, Idx> {
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
        self.edge().points()
    }
    /// returns a mutable reference to the nodes
    pub const fn nodes_mut(&mut self) -> &mut S {
        self.edge_mut().points_mut()
    }
    /// updates the id and returns a mutable reference to the instance
    pub fn set_id(&mut self, id: EdgeId<Idx>) -> &mut Self {
        self.edge_mut().set_id(id);
        self
    }
    /// updates the nodes and returns a mutable reference to the instance
    pub fn set_points(&mut self, nodes: S) -> &mut Self {
        self.edge_mut().set_points(nodes);
        self
    }
    /// updates the weight and returns a mutable reference to the instance
    pub fn set_weight(&mut self, weight: Weight<T>) -> &mut Self {
        self.weight = weight;
        self
    }
    /// consumes the current instance to create another with the given id.
    pub fn with_id(self, id: EdgeId<Idx>) -> Self {
        HyperFacet {
            edge: self.edge.with_id(id),
            weight: self.weight,
        }
    }
    /// consumes the current instance to create another with the given nodes.
    pub fn with_points<S2: RawStore<Idx>>(self, nodes: S2) -> HyperFacet<T, S2, K, Idx> {
        HyperFacet {
            edge: self.edge.with_points(nodes),
            weight: self.weight,
        }
    }
    /// consumes the current instance to create another with the given weight.
    pub fn with_weight<U>(self, weight: Weight<U>) -> HyperFacet<U, S, K, Idx> {
        HyperFacet {
            edge: self.edge,
            weight,
        }
    }
    /// returns true if the edge contains the given vertex
    pub fn contains<Q>(&self, index: &Q) -> bool
    where
        VertexId<Idx>: core::borrow::Borrow<Q>,
        Q: PartialEq,
        Idx: PartialEq,
        for<'a> &'a S: IntoIterator<Item = &'a VertexId<Idx>>,
    {
        self.edge().contains(index)
    }
    /// returns true if the edge is empty
    pub fn is_empty(&self) -> bool {
        self.edge().is_empty()
    }
    /// returns the number of nodes in the edge
    pub fn len(&self) -> usize {
        self.edge().len()
    }
}

impl<T, S, Idx, K> RawEdge for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    type Kind = K;
    type Idx = Idx;
    type Store = S;

    seal!();

    fn index(&self) -> &EdgeId<Idx> {
        self.edge().id()
    }

    fn vertices(&self) -> &S {
        self.edge().points()
    }

    fn vertices_mut(&mut self) -> &mut S {
        self.edge_mut().points_mut()
    }
}

impl<T, S, Idx, K> RawFacet<T> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    seal!();

    fn weight(&self) -> &Weight<T> {
        self.as_ref()
    }

    fn weight_mut(&mut self) -> &mut Weight<T> {
        self.as_mut()
    }
}

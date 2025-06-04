/*
    appellation: impl_hyper_facet <module>
    authors: @FL03
*/
use crate::edge::{HyperEdge, HyperFacet, RawStore};
use crate::index::{EdgeId, RawIndex, VertexId};
use crate::{GraphKind, Weight};

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
    /// returns true if the edge contains the given vertex
    pub fn contains_vertex<Q>(&self, index: &Q) -> bool
    where
        VertexId<Idx>: core::borrow::Borrow<Q>,
        Q: PartialEq,
        Idx: PartialEq,
        for<'a> &'a S: IntoIterator<Item = &'a VertexId<Idx>>,
    {
        self.edge().contains_vertex(index)
    }
    /// returns the number of nodes in the edge
    pub fn len(&self) -> usize {
        self.edge().len()
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
}

impl<T, S, K, Idx> Default for HyperFacet<T, S, K, Idx>
where
    Idx: Default + RawIndex,
    K: GraphKind,
    T: Default,
    S: RawStore<Idx> + Default,
{
    fn default() -> Self {
        Self {
            edge: HyperEdge::default(),
            weight: Weight::default(),
        }
    }
}

impl<T, S, K, Idx> core::fmt::Display for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    T: core::fmt::Display,
    S: RawStore<Idx> + core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ edge: {}, weight: {} }}", self.edge, self.weight)
    }
}

impl<T, S, K, Idx> From<HyperEdge<S, K, Idx>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
    T: Default,
{
    fn from(edge: HyperEdge<S, K, Idx>) -> Self {
        Self::from_edge(edge)
    }
}

impl<T, S, K, Idx> From<HyperFacet<T, S, K, Idx>> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn from(facet: HyperFacet<T, S, K, Idx>) -> Self {
        facet.edge
    }
}

impl<T, S, K, Idx> From<EdgeId<Idx>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: Default + RawStore<Idx>,
    T: Default,
{
    fn from(id: EdgeId<Idx>) -> Self {
        Self::from_id(id)
    }
}

impl<T, S, K, Idx> AsRef<Weight<T>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn as_ref(&self) -> &Weight<T> {
        &self.weight
    }
}

impl<T, S, K, Idx> AsMut<Weight<T>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn as_mut(&mut self) -> &mut Weight<T> {
        &mut self.weight
    }
}

impl<T, S, K, Idx> core::borrow::Borrow<EdgeId<Idx>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn borrow(&self) -> &EdgeId<Idx> {
        self.id()
    }
}

impl<T, S, K, Idx> core::borrow::BorrowMut<EdgeId<Idx>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn borrow_mut(&mut self) -> &mut EdgeId<Idx> {
        self.id_mut()
    }
}

impl<T, S, K, Idx> core::ops::Deref for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    type Target = HyperEdge<S, K, Idx>;

    fn deref(&self) -> &Self::Target {
        self.edge()
    }
}

impl<T, S, K, Idx> core::ops::DerefMut for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.edge_mut()
    }
}

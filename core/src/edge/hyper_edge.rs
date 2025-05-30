/*
    appellation: hyper_edge <module>
    authors: @FL03
*/
use crate::{EdgeId, Weight};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
pub struct Edge<W, S, Idx = usize> {
    pub(crate) id: EdgeId<Idx>,
    pub(crate) nodes: S,
    pub(crate) weight: Weight<W>,
}

impl<W, S, Idx> Edge<W, S, Idx> {
    pub fn new(id: EdgeId<Idx>, nodes: S, weight: W) -> Self {
        Self { id, nodes, weight: Weight(weight) }
    }
    /// creates a new edge with the given id
    pub fn from_id(id: EdgeId<Idx>) -> Self
    where
        S: Default,
        W: Default,
    {
        Self {
            id,
            nodes: Default::default(),
            weight: Default::default(),
        }
    }
    /// creates a new edge with the given nodes
    pub fn from_nodes(nodes: S) -> Self
    where
        Idx: Default,
        W: Default,
    {
        Self {
            id: EdgeId::default(),
            nodes,
            weight: Weight::default(),
        }
    }
    /// creates a new instance from the given value
    pub fn from_weight(weight: Weight<W>) -> Self
    where
        Idx: Default,
        S: Default,
    {
        Self {
            id: EdgeId::default(),
            nodes: S::default(),
            weight,
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
    /// returns an immutable reference to the weight
    pub const fn weight(&self) -> &Weight<W> {
        &self.weight
    }
    /// returns a mutable reference to the weight
    pub const fn weight_mut(&mut self) -> &mut Weight<W> {
        &mut self.weight
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
    /// updates the weight and returns a mutable reference to the instance
    pub fn set_weight(&mut self, weight: Weight<W>) -> &mut Self {
        self.weight = weight;
        self
    }
    /// consumes the current instance to create another with the given id.
    pub fn with_id<I2>(self, id: EdgeId<I2>) -> Edge<W, S, I2> {
        Edge {
            id,
            nodes: self.nodes,
            weight: self.weight,
        }
    }
    /// consumes the current instance to create another with the given nodes.
    pub fn with_nodes<S2>(self, nodes: S2) -> Edge<W, S2, Idx> {
        Edge {
            id: self.id,
            nodes,
            weight: self.weight,
        }
    }
    /// consumes the current instance to create another with the given weight.
    pub fn with_weight<U>(self, weight: Weight<U>) -> Edge<U, S, Idx> {
        Edge {
            id: self.id,
            nodes: self.nodes,
            weight,
        }
    }
}


impl<W, S, Idx> AsRef<Weight<W>> for Edge<W, S, Idx> {
    fn as_ref(&self) -> &Weight<W> {
        &self.weight
    }
}

impl<W, S, Idx> AsMut<Weight<W>> for Edge<W, S, Idx> {
    fn as_mut(&mut self) -> &mut Weight<W> {
        &mut self.weight
    }
}

impl<W, S, Idx> core::borrow::Borrow<EdgeId<Idx>> for Edge<W, S, Idx> {
    fn borrow(&self) -> &EdgeId<Idx> {
        &self.id
    }
}

impl<W, S, Idx> core::borrow::BorrowMut<EdgeId<Idx>> for Edge<W, S, Idx> {
    fn borrow_mut(&mut self) -> &mut EdgeId<Idx> {
        &mut self.id
    }
}

impl<W, S, Idx> core::ops::Deref for Edge<W, S, Idx> {
    type Target = Weight<W>;

    fn deref(&self) -> &Self::Target {
        &self.weight
    }
}

impl<W, S, Idx> core::ops::DerefMut for Edge<W, S, Idx> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.weight
    }
}

impl<W, S, Idx> core::fmt::Display for Edge<W, S, Idx>
where
    Idx: core::fmt::Display,
    W: core::fmt::Display,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Edge(id: {}, nodes: {}, weight: {})", self.id, self.nodes, self.weight)
    }
}
/*
    appellation: hyper_edge <module>
    authors: @FL03
*/
use crate::EdgeId;


#[cfg(feature = "std")]
pub type HashEdge<T, Idx = usize> = Edge<T, std::collections::HashSet<crate::VertexId<Idx>>, Idx>;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
pub struct Edge<T, S, Idx = usize> {
    pub id: EdgeId<Idx>,
    pub nodes: S,
    pub weight: T,
}


impl<T, S, Idx> Edge<T, S, Idx> {
    pub fn new(id: EdgeId<Idx>, nodes: S, weight: T) -> Self {
        Self { id, nodes, weight }
    }

    /// creates a new edge with the given id and default weight
    pub fn from_id(id: EdgeId<Idx>, nodes: S) -> Self
    where
        T: Default,
    {
        Self {
            id,
            nodes,
            weight: T::default(),
        }
    }

    /// creates a new instance from the given value
    pub fn from_weight(weight: T, nodes: S) -> Self
    where
        Idx: Default,
    {
        Self {
            id: EdgeId::default(),
            nodes,
            weight,
        }
    }

    /// consumes the current instance to create another with the given id.
    pub fn with_id<I2>(self, id: EdgeId<I2>) -> Edge<T, S, I2> {
        Edge {
            id,
            nodes: self.nodes,
            weight: self.weight,
        }
    }

    /// consumes the current instance to create another with the given weight.
    pub fn with_weight<U>(self, weight: U) -> Edge<U, S, Idx> {
        Edge {
            id: self.id,
            nodes: self.nodes,
            weight,
        }
    }
}
/*
    Appellation: hgraph <module>
    Contrib: @FL03
*/
use crate::{EdgeId, VertexId};

pub trait KeyValue {
    type Key;
    type Value;
}

pub trait RawNode {
    type Idx;

    private!();

    /// Returns the index of the node.
    fn index(&self) -> &VertexId<Self::Idx>;
}

pub trait RawEdge {
    type Idx;

    private!();

    /// Returns the index of the node.
    fn index(&self) -> &EdgeId<Self::Idx>;
}

pub trait RawHyperGraph {
    type Edge: RawEdge;
    type Node: RawNode;

    private!();
}

/// [`Hypergraph`] is a trait that defines the basic operations for a hypergraph data structure.
pub trait Hypergraph: RawHyperGraph {}

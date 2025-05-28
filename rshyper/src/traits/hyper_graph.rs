/*
    Appellation: hgraph <module>
    Contrib: @FL03
*/


pub trait KeyValue {
    type Key;
    type Value;
}

pub trait RawHyperGraph {
    type Edge: KeyValue;
    type Node: KeyValue;

    private!();
}

/// [`Hypergraph`] is a trait that defines the basic operations for a hypergraph data structure.
pub trait Hypergraph: RawHyperGraph {}

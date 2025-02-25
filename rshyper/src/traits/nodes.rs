/*
    Appellation: nodes <module>
    Contrib: @FL03
*/

/// A trait denoting a node within the hypergraph.
pub trait HyperNode {}

/// Extends the base [HyperNode] trait with the [core::cmp::Eq] and [core::hash::Hash] traits
/// for use with hash-related structures.
pub trait HashNode: HyperNode + core::cmp::Eq + core::hash::Hash {}

/*
 ************* Implementations *************
*/

impl<T> HashNode for T where T: HyperNode + core::cmp::Eq + core::hash::Hash {}

/*
    Appellation: edges <module>
    Contrib: @FL03
*/
use super::{HyperNode, Indexable};

pub trait HyperEdge: Indexable {
    type Node: HyperNode;

    /// Returns true if the hyperedge contains the given vertex
    fn contains(&self, vertex: Self::Idx) -> bool;
    /// Returns the vertices of the hyperedge
    fn vertices(&self) -> &[Self::Node];
}

pub trait HashEdge: HyperEdge + core::cmp::Eq + core::hash::Hash {}

/*
 ************* Implementations *************
*/
impl<T> HashEdge for T where T: HyperEdge + core::cmp::Eq + core::hash::Hash {}

/*
    appellation: priority_node <module>
    authors: @FL03
*/
use core::cmp::Ordering;
use rshyper::idx::{RawIndex, VertexId};

/// Priority queue node for A* algorithm
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PriorityNode<P = i64, Idx = usize>
where
    Idx: RawIndex,
{
    pub(crate) vertex: VertexId<Idx>,
    pub(crate) priority: P, // Negative f_score for min-heap behavior
}

impl<P, Idx> PriorityNode<P, Idx>
where
    Idx: RawIndex,
{
    /// Create a new priority node with the given vertex and priority
    pub fn new(vertex: VertexId<Idx>, priority: P) -> Self {
        Self { vertex, priority }
    }
    /// returns an immutable reference to the priority of the node
    pub const fn priority(&self) -> &P {
        &self.priority
    }
    /// returns an immutable reference to the vertex of the node
    pub const fn vertex(&self) -> &VertexId<Idx> {
        &self.vertex
    }
}

impl<P, Idx> PartialEq<P> for PriorityNode<P, Idx>
where
    P: PartialEq,
    Idx: RawIndex + PartialEq,
{
    fn eq(&self, other: &P) -> bool {
        self.priority() == other
    }
}

impl<P, Idx> Ord for PriorityNode<P, Idx>
where
    P: Ord,
    Idx: RawIndex + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering to create a min-heap (lowest f_score has highest priority)
        other.priority().cmp(&self.priority)
    }
}

impl<P, Idx> PartialOrd for PriorityNode<P, Idx>
where
    P: PartialOrd,
    Idx: RawIndex + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse ordering to create a min-heap (lowest f_score has highest priority)
        other.priority().partial_cmp(&self.priority)
    }
}

impl<P, Idx> PartialOrd<P> for PriorityNode<P, Idx>
where
    P: PartialOrd,
    Idx: RawIndex + PartialOrd,
{
    fn partial_cmp(&self, other: &P) -> Option<Ordering> {
        // Reverse ordering to create a min-heap (lowest f_score has highest priority)
        other.partial_cmp(self.priority())
    }
}

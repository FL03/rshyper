/*
    appellation: priority_node <module>
    authors: @FL03
*/
use crate::VertexId;
use core::cmp::Ordering;

/// Priority queue node for A* algorithm
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PriorityNode<P = i64> {
    pub(crate) vertex: VertexId,
    pub(crate) priority: P, // Negative f_score for min-heap behavior
}

impl<P> PriorityNode<P> {
    /// Create a new priority node with the given vertex and priority
    pub fn new(vertex: VertexId, priority: P) -> Self {
        Self { vertex, priority }
    }
    /// returns an immutable reference to the priority of the node
    pub const fn priority(&self) -> &P {
        &self.priority
    }
    /// returns a copy of the associated vertex index
    pub const fn vertex(&self) -> VertexId {
        self.vertex
    }
}

impl<P> PartialEq<P> for PriorityNode<P>
where
    P: PartialEq,
{
    fn eq(&self, other: &P) -> bool {
        self.priority() == other
    }
}

impl<P> Ord for PriorityNode<P>
where
    P: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering to create a min-heap (lowest f_score has highest priority)
        other.priority().cmp(&self.priority)
    }
}

impl<P> PartialOrd for PriorityNode<P>
where
    P: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority().partial_cmp(other.priority())
    }
}

impl<P> PartialOrd<P> for PriorityNode<P>
where
    P: PartialOrd,
{
    fn partial_cmp(&self, other: &P) -> Option<Ordering> {
        self.priority().partial_cmp(other)
    }
}





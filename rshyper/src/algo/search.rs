/*
    Appellation: search <module>
    Contrib: @FL03
*/

mod impl_astar;
mod impl_bft;
mod impl_dft;

use crate::{HyperGraph, VertexId};
use std::collections::{HashSet, VecDeque};

/// Breadth-First Traversal algorithm for hypergraphs
pub struct BreadthFirstTraversal<'a, N> {
    pub(crate) graph: &'a HyperGraph<N>,
    pub(crate) queue: VecDeque<VertexId>,
    pub(crate) visited: HashSet<VertexId>,
}

/// A* Search algorithm for hypergraphs
pub struct AStarSearch<'a, N, F>
where
    F: Fn(VertexId, VertexId) -> f64,
{
    pub(crate) graph: &'a HyperGraph<N>,
    pub(crate) open_set: HashSet<VertexId>,
    pub(crate) closed_set: HashSet<VertexId>,
    pub(crate) came_from: std::collections::HashMap<VertexId, VertexId>,
    pub(crate) g_score: std::collections::HashMap<VertexId, f64>,
    pub(crate) f_score: std::collections::HashMap<VertexId, f64>,
    pub(crate) heuristic: F,
}

/// Depth-First Traversal algorithm for hypergraphs
pub struct DepthFirstTraversal<'a, N> {
    pub(crate) graph: &'a HyperGraph<N>,
    pub(crate) stack: Vec<VertexId>,
    pub(crate) visited: HashSet<VertexId>,
}

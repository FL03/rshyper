/*
    Appellation: search <module>
    Contrib: @FL03
*/

mod impl_astar;
mod impl_bft;
mod impl_dft;

use crate::{HashGraph, VertexId};
#[cfg(feature = "alloc")]
use alloc::{collections::VecDeque, vec::Vec};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

/// Breadth-First Traversal algorithm for hypergraphs
pub struct BreadthFirstTraversal<'a, N, E> {
    pub(crate) graph: &'a HashGraph<N, E>,
    pub(crate) queue: VecDeque<VertexId>,
    pub(crate) visited: HashSet<VertexId>,
}

/// A* Search algorithm for hypergraphs
pub struct AStarSearch<'a, N, E, F>
where
    F: Fn(VertexId, VertexId) -> f64,
{
    pub(crate) graph: &'a HashGraph<N, E>,
    pub(crate) open_set: HashSet<VertexId>,
    pub(crate) closed_set: HashSet<VertexId>,
    pub(crate) came_from: HashMap<VertexId, VertexId>,
    pub(crate) g_score: HashMap<VertexId, f64>,
    pub(crate) f_score: HashMap<VertexId, f64>,
    pub(crate) heuristic: F,
}

/// Depth-First Traversal algorithm for hypergraphs
pub struct DepthFirstTraversal<'a, N, E> {
    pub(crate) graph: &'a HashGraph<N, E>,
    pub(crate) stack: Vec<VertexId>,
    pub(crate) visited: HashSet<VertexId>,
}

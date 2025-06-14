/*
    appellation: dijkstra <module>
    authors: @FL03
*/
#[doc(inline)]
pub use self::queue_node::QueueNode;
/// this module implements the queue node used in Dijkstra's algorithm
mod queue_node;

use crate::algo::{PathFinder, Search, Traversal};
use crate::hash_graph::HashGraph;
use core::hash::Hash;
use rshyper_core::idx::{NumIndex, RawIndex, VertexId};
use rshyper_core::{GraphAttributes, GraphType, HyperGraph};
use std::collections::{BinaryHeap, HashSet};

pub(crate) type Distances<K, V = f64> = std::collections::HashMap<VertexId<K>, V>;

pub(crate) type PreviousHistory<K> = std::collections::HashMap<VertexId<K>, VertexId<K>>;

pub(crate) type Visited<K> = std::collections::HashSet<VertexId<K>>;

/// Dijkstra's shortest path algorithm for hypergraphs
pub struct Dijkstra<'a, N, E, A, H = HashGraph<N, E, A>>
where
    A: GraphAttributes,
    H: HyperGraph<N, E, A>,
{
    pub(crate) graph: &'a H,
    pub(crate) distances: Distances<A::Ix, f64>,
    pub(crate) previous: PreviousHistory<A::Ix>,
    pub(crate) visited: Visited<A::Ix>,
    _marker: core::marker::PhantomData<(N, E)>,
}

impl<'a, N, E, A, H, K, Idx> Dijkstra<'a, N, E, A, H>
where
    E: Eq + Hash,
    N: Eq + Hash,
    A: GraphAttributes<Ix = Idx, Kind = K>,
    H: HyperGraph<N, E, A>,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
{
    /// Create a new Dijkstra instance
    pub fn new(graph: &'a H) -> Self {
        Self {
            graph,
            distances: Distances::new(),
            previous: PreviousHistory::new(),
            visited: Visited::new(),
            _marker: core::marker::PhantomData::<(N, E)>,
        }
    }
    /// returns a reference to the graph
    pub const fn graph(&self) -> &H {
        self.graph
    }
    /// returns a reference to the distances
    pub const fn distances(&self) -> &Distances<Idx, f64> {
        &self.distances
    }
    /// returns a mutable reference to the distances
    pub const fn distances_mut(&mut self) -> &mut Distances<Idx, f64> {
        &mut self.distances
    }
    /// returns a reference to the previous history
    pub const fn previous(&self) -> &PreviousHistory<Idx> {
        &self.previous
    }
    /// returns a mutable reference to the previous history
    pub const fn previous_mut(&mut self) -> &mut PreviousHistory<Idx> {
        &mut self.previous
    }
    /// returns a reference to the visited vertices
    pub const fn visited(&self) -> &Visited<Idx> {
        &self.visited
    }
    /// returns a mutable reference to the visited vertices
    pub const fn visited_mut(&mut self) -> &mut Visited<Idx> {
        &mut self.visited
    }
    /// update the distances and returns a mutable reference to the instance
    pub fn set_distances(&mut self, distances: Distances<Idx, f64>) -> &mut Self {
        *self.distances_mut() = distances;
        self
    }
    /// update the previous history and returns a mutable reference to the instance
    pub fn set_previous(&mut self, previous: PreviousHistory<Idx>) -> &mut Self {
        *self.previous_mut() = previous;
        self
    }
    /// update the visited vertices and returns a mutable reference to the instance
    pub fn set_visited(&mut self, visited: Visited<Idx>) -> &mut Self {
        *self.visited_mut() = visited;
        self
    }
    /// add a new node to the distances
    pub fn add_distance(&mut self, vertex: VertexId<Idx>, distance: f64) -> &mut Self {
        self.distances_mut().insert(vertex, distance);
        self
    }
    /// add a vertex to the previous history
    pub fn add_previous(&mut self, vertex: VertexId<Idx>, previous: VertexId<Idx>) -> &mut Self {
        self.previous_mut().insert(vertex, previous);
        self
    }
    /// record a vertex as visited
    pub fn add_visited(&mut self, vertex: VertexId<Idx>) -> &mut Self {
        self.visited_mut().insert(vertex);
        self
    }
    /// Find the shortest path from `start` to `goal`
    pub fn find_path(
        &mut self,
        start: VertexId<Idx>,
        dest: VertexId<Idx>,
    ) -> crate::Result<<Self as PathFinder<VertexId<Idx>>>::Path>
    where
        Self: PathFinder<VertexId<Idx>>,
    {
        PathFinder::find_path(self, start, dest)
    }
    /// search for a path starting from `start` to the vertex with the largest ID
    pub fn search(
        &mut self,
        start: VertexId<Idx>,
    ) -> crate::Result<<Self as Search<VertexId<Idx>>>::Output>
    where
        Self: Search<VertexId<Idx>>,
    {
        Search::search(self, start)
    }
    /// returns true if the vertex is visited
    pub fn has_visited<Q>(&self, vertex: &Q) -> bool
    where
        Q: ?Sized + Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.visited().contains(vertex)
    }
    /// Reset the state for reuse
    pub fn reset(&mut self) -> &mut Self {
        self.distances_mut().clear();
        self.previous_mut().clear();
        self.visited_mut().clear();
        self
    }
}

impl<'a, N, E, A, S, K, Idx> PathFinder<VertexId<Idx>>
    for Dijkstra<'a, N, E, A, HashGraph<N, E, A, S>>
where
    E: Eq + Hash,
    N: Eq + Hash,
    A: GraphAttributes<Ix = Idx, Kind = K>,
    S: core::hash::BuildHasher + Default,
    K: GraphType,
    Idx: NumIndex,
{
    type Path = Vec<VertexId<Idx>>;

    fn find_path(&mut self, src: VertexId<Idx>, dest: VertexId<Idx>) -> crate::Result<Self::Path> {
        self.reset();

        if !self.graph.contains_node(&src) {
            return Err(crate::Error::NodeNotFound);
        }
        if !self.graph.contains_node(&dest) {
            return Err(crate::Error::NodeNotFound);
        }

        let mut heap = BinaryHeap::new();
        self.add_distance(src, 0.0);
        heap.push(QueueNode::new(0.0, src));

        while let Some(QueueNode {
            vertex: u,
            cost: u_cost,
        }) = heap.pop()
        {
            // Only mark as visited when popping from the heap
            if self.has_visited(&u) {
                continue;
            }
            self.add_visited(u);

            if u == dest {
                return Ok(self.reconstruct_path(dest));
            }

            // For each neighbor via hyperedges
            if let Ok(edges) = self.graph().find_edges_with_node(&u) {
                for edge_id in edges {
                    for v in self.graph.get_edge_vertices(&edge_id)?.iter().copied() {
                        if v == u {
                            continue;
                        }
                        let edge_weight = 1.0; // Replace with actual edge weight if available
                        let alt = u_cost + edge_weight;
                        if alt < *self.distances().get(&v).unwrap_or(&f64::INFINITY) {
                            self.add_distance(v, alt).add_previous(v, u);
                            heap.push(QueueNode::new(alt, v));
                        }
                    }
                }
            }
        }

        Err(crate::Error::PathNotFound)
    }

    fn reconstruct_path(&self, mut goal: VertexId<Idx>) -> Vec<VertexId<Idx>> {
        // initialize a new path buffer
        let mut path = Vec::new();
        // add the target
        path.push(goal);
        // reconstruct the path by following the previous vertices
        while let Some(&prev) = self.previous().get(&goal) {
            path.push(prev);
            goal = prev;
        }
        // reverse the path
        path.reverse();
        // return the reconstructed path
        path
    }
}

impl<'a, N, E, A, H> Traversal<VertexId<A::Ix>> for Dijkstra<'a, N, E, A, H>
where
    A: GraphAttributes,
    H: HyperGraph<N, E, A>,
    A::Ix: Eq + Hash,
{
    type Store<I2> = HashSet<I2>;

    fn has_visited(&self, vertex: &VertexId<A::Ix>) -> bool {
        self.visited().contains(vertex)
    }

    fn visited(&self) -> &Self::Store<VertexId<A::Ix>> {
        &self.visited
    }
}

impl<'a, N, E, A, S> Search<VertexId<A::Ix>> for Dijkstra<'a, N, E, A, HashGraph<N, E, A, S>>
where
    E: Eq + Hash,
    N: Eq + Hash,
    A: GraphAttributes,
    S: core::hash::BuildHasher + Default,
    A::Ix: NumIndex,
{
    type Output = Vec<VertexId<A::Ix>>;

    fn search(&mut self, start: VertexId<A::Ix>) -> crate::Result<Self::Output> {
        // Use the vertex with the largest ID as a pseudo-goal if not specified
        let max_vertex_id = match self.graph.nodes().keys().max() {
            Some(&id) => id,
            None => return Ok(vec![]),
        };
        self.find_path(start, max_vertex_id)
    }
}

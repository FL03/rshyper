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
use num_traits::bounds::UpperBounded;
use num_traits::{FromPrimitive, Num};
use rshyper_core::idx::{NumIndex, RawIndex, VertexId};
use rshyper_core::{GraphAttributes, HyperGraph};
use std::collections::{BinaryHeap, HashSet};
/// a type alias for a map of distances for vertices in the graph
pub(crate) type Distances<K, V = f64> = std::collections::HashMap<VertexId<K>, V>;
/// a type alias for the history of previous vertices in the graph, maps vertices to vertices
pub(crate) type PreviousHistory<K> = std::collections::HashMap<VertexId<K>, VertexId<K>>;
/// a type alias for a [`HashSet`](std::collections::HashSet) of visited vertices
pub(crate) type Visited<K> = std::collections::HashSet<VertexId<K>>;

/// Dijkstra's shortest path algorithm for hypergraphs
pub struct Dijkstra<'a, N, E, A, H = HashGraph<N, E, A>>
where
    A: GraphAttributes,
    H: HyperGraph<N, E, A>,
{
    pub(crate) graph: &'a H,
    pub(crate) distances: Distances<A::Ix, E>,
    pub(crate) previous: PreviousHistory<A::Ix>,
    pub(crate) visited: Visited<A::Ix>,
    _marker: core::marker::PhantomData<(N, E)>,
}

impl<'a, N, E, A, H> Dijkstra<'a, N, E, A, H>
where
    A: GraphAttributes,
    H: HyperGraph<N, E, A>,
    A::Ix: RawIndex,
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
    pub const fn distances(&self) -> &Distances<A::Ix, E> {
        &self.distances
    }
    /// returns a mutable reference to the distances
    pub const fn distances_mut(&mut self) -> &mut Distances<A::Ix, E> {
        &mut self.distances
    }
    /// returns a reference to the previous history
    pub const fn previous(&self) -> &PreviousHistory<A::Ix> {
        &self.previous
    }
    /// returns a mutable reference to the previous history
    pub const fn previous_mut(&mut self) -> &mut PreviousHistory<A::Ix> {
        &mut self.previous
    }
    /// returns a reference to the visited vertices
    pub const fn visited(&self) -> &Visited<A::Ix> {
        &self.visited
    }
    /// returns a mutable reference to the visited vertices
    pub const fn visited_mut(&mut self) -> &mut Visited<A::Ix> {
        &mut self.visited
    }
    /// update the distances and returns a mutable reference to the instance
    pub fn set_distances(&mut self, distances: Distances<A::Ix, E>) -> &mut Self {
        *self.distances_mut() = distances;
        self
    }
    /// update the previous history and returns a mutable reference to the instance
    pub fn set_previous(&mut self, previous: PreviousHistory<A::Ix>) -> &mut Self {
        *self.previous_mut() = previous;
        self
    }
    /// update the visited vertices and returns a mutable reference to the instance
    pub fn set_visited(&mut self, visited: Visited<A::Ix>) -> &mut Self {
        *self.visited_mut() = visited;
        self
    }
    /// add a new node to the distances
    pub fn add_distance(&mut self, vertex: VertexId<A::Ix>, distance: E) -> &mut Self
    where
        A::Ix: Eq + Hash,
    {
        self.distances_mut().insert(vertex, distance);
        self
    }
    /// add a vertex to the previous history
    pub fn add_previous(&mut self, vertex: VertexId<A::Ix>, previous: VertexId<A::Ix>) -> &mut Self
    where
        A::Ix: Eq + Hash,
    {
        self.previous_mut().insert(vertex, previous);
        self
    }
    /// record a vertex as visited
    pub fn add_visited(&mut self, vertex: VertexId<A::Ix>) -> &mut Self
    where
        A::Ix: Eq + Hash,
    {
        self.visited_mut().insert(vertex);
        self
    }
    /// Find the shortest path from `start` to `goal`
    pub fn find_path(
        &mut self,
        start: VertexId<A::Ix>,
        dest: VertexId<A::Ix>,
    ) -> crate::Result<<Self as PathFinder<A::Ix>>::Path>
    where
        Self: PathFinder<A::Ix>,
    {
        PathFinder::find_path(self, start, dest)
    }
    /// search for a path starting from `start` to the vertex with the largest ID
    pub fn search(
        &mut self,
        start: VertexId<A::Ix>,
    ) -> crate::Result<<Self as Search<VertexId<A::Ix>>>::Output>
    where
        Self: Search<VertexId<A::Ix>>,
    {
        Search::search(self, start)
    }
    /// returns true if the vertex is visited
    pub fn has_visited<Q>(&self, vertex: &Q) -> bool
    where
        Q: ?Sized + Eq + Hash,
        A::Ix: Eq + Hash,
        VertexId<A::Ix>: core::borrow::Borrow<Q>,
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

impl<'a, N, E, A, S> PathFinder<A::Ix> for Dijkstra<'a, N, E, A, HashGraph<N, E, A, S>>
where
    E: Copy + Default + PartialOrd + FromPrimitive + Num + UpperBounded,
    A: GraphAttributes,
    S: core::hash::BuildHasher + Default,
    A::Ix: NumIndex,
{
    type Path = Vec<VertexId<A::Ix>>;

    fn find_path(
        &mut self,
        src: VertexId<A::Ix>,
        dest: VertexId<A::Ix>,
    ) -> crate::Result<Self::Path> {
        self.reset();

        if !self.graph.contains_node(&src) {
            return Err(crate::Error::NodeNotFound);
        }
        if !self.graph.contains_node(&dest) {
            return Err(crate::Error::NodeNotFound);
        }

        let mut heap: BinaryHeap<QueueNode<A::Ix, E>> = BinaryHeap::new();
        self.add_distance(src, E::zero());
        heap.push(QueueNode::from_vertex(src));

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
                    // load the weight of the edge
                    let weight = self.graph.get_edge_weight(&edge_id)?;
                    // visit each node within the hyperedge
                    for v in self.graph.get_edge_vertices(&edge_id)?.iter().copied() {
                        if v == u {
                            continue;
                        }
                        let alt = u_cost + **weight;
                        if alt < *self.distances().get(&v).unwrap_or(&E::max_value()) {
                            self.add_distance(v, alt).add_previous(v, u);
                            heap.push(QueueNode::new(alt, v));
                        }
                    }
                }
            }
        }

        Err(crate::Error::PathNotFound)
    }

    fn reconstruct_path(&self, mut goal: VertexId<A::Ix>) -> Vec<VertexId<A::Ix>> {
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
    E: Copy + Default + PartialOrd + FromPrimitive + Num + UpperBounded,
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

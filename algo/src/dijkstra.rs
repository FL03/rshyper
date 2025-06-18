/*
    appellation: dijkstra <module>
    authors: @FL03
*/
//! this module implements Dijkstra's shortest-path algorithm for hypergraphs
#[doc(inline)]
pub use self::queue_node::QueueNode;

mod queue_node;

use crate::error::{Error, Result};
use crate::types::{HashMap, HashSet, VertexSet};
use crate::{DefaultHashBuilder, PathFinder, Search, Traversal};
use core::hash::{BuildHasher, Hash};
use num_traits::bounds::UpperBounded;
use num_traits::{FromPrimitive, Num};
use rshyper::idx::{NumIndex, RawIndex, VertexId};
use rshyper::rel::RawLayout;
use rshyper::{GraphProps, HyperGraph, HyperGraphIter};
use std::collections::BinaryHeap;

/// a type alias for a map of distances for vertices in the graph
pub(crate) type Distances<K, V = f64, S = DefaultHashBuilder> = HashMap<VertexId<K>, V, S>;
/// a type alias for the history of previous vertices in the graph, maps vertices to vertices
pub(crate) type PreviousHistory<K, S = DefaultHashBuilder> = HashMap<VertexId<K>, VertexId<K>, S>;

/// Dijkstra's shortest path algorithm for hypergraphs
pub struct Dijkstra<'a, N, E, A, H, S = DefaultHashBuilder>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
{
    pub(crate) graph: &'a H,
    pub(crate) distances: Distances<A::Ix, E, S>,
    pub(crate) previous: PreviousHistory<A::Ix, S>,
    pub(crate) visited: VertexSet<A::Ix, S>,
    _marker: core::marker::PhantomData<(N, E)>,
}

impl<'a, N, E, A, H, S> Dijkstra<'a, N, E, A, H, S>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
    S: BuildHasher,
    A::Ix: RawIndex,
{
    /// Create a new Dijkstra instance
    pub fn new(graph: &'a H) -> Self
    where
        S: Default,
    {
        Self {
            graph,
            distances: Distances::default(),
            previous: PreviousHistory::default(),
            visited: VertexSet::default(),
            _marker: core::marker::PhantomData::<(N, E)>,
        }
    }
    /// returns a reference to the graph
    pub const fn graph(&self) -> &H {
        self.graph
    }
    /// returns a reference to the distances
    pub const fn distances(&self) -> &Distances<A::Ix, E, S> {
        &self.distances
    }
    /// returns a mutable reference to the distances
    pub const fn distances_mut(&mut self) -> &mut Distances<A::Ix, E, S> {
        &mut self.distances
    }
    /// returns a reference to the previous history
    pub const fn previous(&self) -> &PreviousHistory<A::Ix, S> {
        &self.previous
    }
    /// returns a mutable reference to the previous history
    pub const fn previous_mut(&mut self) -> &mut PreviousHistory<A::Ix, S> {
        &mut self.previous
    }
    /// returns a reference to the visited vertices
    pub const fn visited(&self) -> &VertexSet<A::Ix, S> {
        &self.visited
    }
    /// returns a mutable reference to the visited vertices
    pub const fn visited_mut(&mut self) -> &mut VertexSet<A::Ix, S> {
        &mut self.visited
    }
    /// update the distances and returns a mutable reference to the instance
    pub fn set_distances(&mut self, distances: Distances<A::Ix, E, S>) -> &mut Self {
        *self.distances_mut() = distances;
        self
    }
    /// update the previous history and returns a mutable reference to the instance
    pub fn set_previous(&mut self, previous: PreviousHistory<A::Ix, S>) -> &mut Self {
        *self.previous_mut() = previous;
        self
    }
    /// update the visited vertices and returns a mutable reference to the instance
    pub fn set_visited(&mut self, visited: VertexSet<A::Ix, S>) -> &mut Self {
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
    ) -> Result<<Self as PathFinder<A::Ix>>::Path>
    where
        Self: PathFinder<A::Ix>,
    {
        PathFinder::find_path(self, start, dest)
    }
    /// search for a path starting from `start` to the vertex with the largest ID
    pub fn search(
        &mut self,
        start: VertexId<A::Ix>,
    ) -> Result<<Self as Search<VertexId<A::Ix>>>::Output>
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

impl<'a, N, E, A, H, S> PathFinder<A::Ix> for Dijkstra<'a, N, E, A, H, S>
where
    E: Copy + Default + PartialOrd + FromPrimitive + Num + UpperBounded,
    A: GraphProps,
    H: HyperGraph<N, E, A>,
    S: BuildHasher,
    A::Ix: NumIndex,
    <H::Edge<E> as RawLayout>::Store: Clone + IntoIterator<Item = VertexId<A::Ix>>,
{
    type Path = Vec<VertexId<A::Ix>>;

    fn find_path(&mut self, src: VertexId<A::Ix>, dest: VertexId<A::Ix>) -> Result<Self::Path> {
        self.reset();

        if !self.graph.contains_node(&src) {
            return Err(Error::NodeNotFound);
        }
        if !self.graph.contains_node(&dest) {
            return Err(Error::NodeNotFound);
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
            for edge_id in self.graph.find_edges_with_node(&u) {
                // load the weight of the edge
                let weight = self
                    .graph
                    .get_edge_weight(&edge_id)
                    .expect("no weight for the edge")
                    .view();
                // visit each node within the hyperedge
                for v in self
                    .graph
                    .get_edge_domain(&edge_id)
                    .expect("empty hyperedge")
                    .clone()
                {
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
        Err(Error::PathNotFound)
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

impl<'a, N, E, A, H, S> Traversal<VertexId<A::Ix>> for Dijkstra<'a, N, E, A, H, S>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
    S: BuildHasher,
    A::Ix: Eq + Hash,
{
    type Store<I2> = HashSet<I2, S>;

    fn has_visited(&self, vertex: &VertexId<A::Ix>) -> bool {
        self.visited().contains(vertex)
    }

    fn visited(&self) -> &Self::Store<VertexId<A::Ix>> {
        &self.visited
    }
}

impl<'a, N, E, A, H> Search<VertexId<A::Ix>> for Dijkstra<'a, N, E, A, H>
where
    E: Copy + Default + PartialOrd + FromPrimitive + Num + UpperBounded,
    A: GraphProps,
    H: HyperGraphIter<N, E, A>,
    A::Ix: NumIndex,
    <H::Edge<E> as RawLayout>::Store: Clone + IntoIterator<Item = VertexId<A::Ix>>,
{
    type Output = Vec<VertexId<A::Ix>>;

    fn search(&mut self, start: VertexId<A::Ix>) -> Result<Self::Output> {
        // Use the vertex with the largest ID as a pseudo-goal if not specified
        let max_vertex_id = match self.graph.vertices().max() {
            Some(&id) => id,
            None => {
                #[cfg(feature = "tracing")]
                tracing::warn!("Graph is empty, returning an empty path.");
                return Ok(Vec::new());
            }
        };
        // use the path-finding algorithm to find the path
        let path = self.find_path(start, max_vertex_id)?;
        #[cfg(feature = "tracing")]
        tracing::info!("found path: {:?}", path);
        Ok(path)
    }
}

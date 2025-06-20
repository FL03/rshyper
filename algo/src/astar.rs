/*
    Appellation: impl_astar <module>
    Contrib: @FL03
*/
//! this module implements the A* search algorithm
#[doc(inline)]
pub use self::priority_node::PriorityNode;

mod priority_node;

use crate::error::{Error, Result};
use crate::traits::{Heuristic, PathFinder, Search, Traversal};
use alloc::collections::BinaryHeap;
use core::hash::{BuildHasher, Hash};
use hashbrown::{DefaultHashBuilder, HashMap, HashSet};
use rshyper::idx::{HyperIndex, RawIndex, VertexId, VertexSet};
use rshyper::rel::RawLayout;
use rshyper::{GraphProps, GraphType, HyperGraph, HyperGraphIter};

type SourceMap<Ix, S = DefaultHashBuilder> = HashMap<VertexId<Ix>, VertexId<Ix>, S>;

type ScoreMap<K, V, S = DefaultHashBuilder> = HashMap<VertexId<K>, V, S>;

/// An A* Search algorithm implementation for hypergraphs
pub struct AStarSearch<'a, N, E, A, F, H, S = DefaultHashBuilder>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
    F: Heuristic<A::Ix>,
{
    pub(crate) graph: &'a H,
    pub(crate) open_set: VertexSet<A::Ix, S>,
    pub(crate) closed_set: VertexSet<A::Ix, S>,
    pub(crate) came_from: SourceMap<A::Ix, S>,
    pub(crate) g_score: ScoreMap<A::Ix, F::Output, S>,
    pub(crate) f_score: ScoreMap<A::Ix, F::Output, S>,
    pub(crate) heuristic: F,
    _marker: core::marker::PhantomData<(N, E)>,
}

impl<'a, N, E, A, F, H, S, K, Idx> AStarSearch<'a, N, E, A, F, H, S>
where
    A: GraphProps<Ix = Idx, Kind = K>,
    H: HyperGraph<N, E, A>,
    F: Heuristic<Idx>,
    S: BuildHasher,
    K: GraphType,
    Idx: RawIndex,
{
    /// Create a new A* search instance with the given heuristic function
    pub fn new(graph: &'a H, heuristic: F) -> Self
    where
        S: Default,
    {
        Self {
            heuristic,
            graph,
            open_set: VertexSet::default(),
            closed_set: VertexSet::default(),
            came_from: SourceMap::default(),
            g_score: ScoreMap::default(),
            f_score: ScoreMap::default(),
            _marker: core::marker::PhantomData::<(N, E)>,
        }
    }
    /// consumes the current instance to create another from the given heuristic function;
    /// **note:** while the functions may be different, the output type of both must match.
    pub fn with_heuristic<G>(self, heuristic: G) -> AStarSearch<'a, N, E, A, G, H, S>
    where
        G: Heuristic<Idx, Output = F::Output>,
    {
        AStarSearch {
            heuristic,
            graph: self.graph,
            open_set: self.open_set,
            closed_set: self.closed_set,
            came_from: self.came_from,
            g_score: self.g_score,
            f_score: self.f_score,
            _marker: self._marker,
        }
    }

    pub const fn came_from(&self) -> &SourceMap<A::Ix, S> {
        &self.came_from
    }
    /// returns a mutable reference to the map of vertices that have been processed
    pub const fn came_from_mut(&mut self) -> &mut SourceMap<A::Ix, S> {
        &mut self.came_from
    }
    /// returns an immutable reference to the closed set of vertices
    pub const fn closed_set(&self) -> &VertexSet<A::Ix, S> {
        &self.closed_set
    }
    /// returns a mutable reference to the closed set of vertices
    pub const fn closed_set_mut(&mut self) -> &mut VertexSet<A::Ix, S> {
        &mut self.closed_set
    }
    /// returns an immutable reference to the f_score map
    pub const fn f_score(&self) -> &ScoreMap<A::Ix, F::Output, S> {
        &self.f_score
    }
    /// returns a mutable reference to the f_score map
    pub const fn f_score_mut(&mut self) -> &mut ScoreMap<A::Ix, F::Output, S> {
        &mut self.f_score
    }
    /// returns an immutable reference to the g_score map
    pub const fn g_score(&self) -> &ScoreMap<A::Ix, F::Output, S> {
        &self.g_score
    }
    /// returns a mutable reference to the g_score map
    pub const fn g_score_mut(&mut self) -> &mut ScoreMap<A::Ix, F::Output, S> {
        &mut self.g_score
    }
    /// returns an immutable reference to the heuristic function of the algorithm
    pub const fn heuristic(&self) -> &F {
        &self.heuristic
    }
    /// returns an immutable reference to the set of vertices that have been visited
    pub const fn open_set(&self) -> &VertexSet<A::Ix, S> {
        &self.open_set
    }
    /// returns amutable reference to the open set of vertices
    pub const fn open_set_mut(&mut self) -> &mut VertexSet<A::Ix, S> {
        &mut self.open_set
    }
    /// returns true if the given vertex has a f_score
    pub fn has_f_score<Q>(&self, vertex: &Q) -> bool
    where
        Q: ?Sized + Eq + Hash,
        Idx: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.f_score().contains_key(vertex)
    }
    /// returns true if the given vertex has a g_score
    pub fn has_g_score<Q>(&self, vertex: &Q) -> bool
    where
        Q: ?Sized + Eq + Hash,
        Idx: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.g_score().contains_key(vertex)
    }
    /// returns true if the given vertex has been visited
    pub fn has_visited<Q>(&self, vertex: &Q) -> bool
    where
        Q: ?Sized + Eq + Hash,
        Idx: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.closed_set().contains(vertex)
    }
    /// returns true if the given vertex is in the open set
    pub fn in_open_set<Q>(&self, vertex: &Q) -> bool
    where
        Q: ?Sized + Eq + Hash,
        Idx: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.open_set().contains(vertex)
    }
    /// moves the vertex from the open set before inserting it into the closed set; this is
    /// useful for updating the state, marking a node as processed.
    pub fn move_open_to_closed(&mut self, vertex: &VertexId<Idx>)
    where
        Idx: Copy + Eq + Hash,
    {
        self.open_set_mut().remove(vertex);
        self.closed_set_mut().insert(*vertex);
    }
    /// reset the state
    pub fn reset(&mut self) -> &mut Self {
        self.open_set_mut().clear();
        self.closed_set_mut().clear();
        self.came_from_mut().clear();
        self.g_score_mut().clear();
        self.f_score_mut().clear();
        self
    }
    /// find a path between two nodes
    pub fn find_path(
        &mut self,
        start: VertexId<Idx>,
        goal: VertexId<Idx>,
    ) -> Result<<Self as PathFinder<Idx>>::Path>
    where
        Self: PathFinder<Idx>,
    {
        PathFinder::find_path(self, start, goal)
    }
    /// a convience method to perform a search
    pub fn search(
        &mut self,
        start: VertexId<Idx>,
    ) -> Result<<Self as Search<VertexId<Idx>>>::Output>
    where
        Self: Search<VertexId<Idx>>,
    {
        Search::search(self, start)
    }
}

impl<'a, N, E, F, A, H, S> PathFinder<A::Ix> for AStarSearch<'a, N, E, A, F, H, S>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
    F: Heuristic<A::Ix, Output = f64>,
    S: BuildHasher,
    A::Ix: HyperIndex,
    for<'b> &'b <H::Edge<E> as RawLayout>::Store: IntoIterator<Item = &'b VertexId<A::Ix>>,
{
    type Path = Vec<VertexId<A::Ix>>;
    /// Find the shortest path between start and goal vertices
    fn find_path(&mut self, start: VertexId<A::Ix>, goal: VertexId<A::Ix>) -> Result<Self::Path> {
        // Check if both vertices exist
        if !self.graph.contains_node(&start) {
            return Err(rshyper::Error::NodeNotFound.into());
        }
        if !self.graph.contains_node(&goal) {
            return Err(rshyper::Error::NodeNotFound.into());
        }

        // reset state
        self.reset();
        // initialize g_score for start node (0) and infinity for all other nodes
        self.g_score_mut().insert(start, 0.0);

        // initialize f_score for start node (heuristic only since g=0)
        let initial_fscore = self.heuristic().compute(start, goal);
        self.f_score_mut().insert(start, initial_fscore);
        // add start node to the open set
        self.open_set_mut().insert(start);
        // initialize priority queue
        let mut priority_queue = BinaryHeap::new();
        // push the start node with its f_score
        priority_queue.push(PriorityNode {
            vertex: start,
            priority: -(initial_fscore as i64),
        });
        // track processed nodes to avoid duplicate processing
        let mut processed = HashSet::new();
        // process nodes until the queue is empty or we attain the goal
        while let Some(PriorityNode {
            vertex: current, ..
        }) = priority_queue.pop()
        {
            // Skip if we've already processed this vertex with a better path
            // or it's no longer in the open set
            if processed.contains(&current) || !self.in_open_set(&current) {
                continue;
            }
            // add the current vertex to the processed set
            processed.insert(current);

            // If we've reached the goal, construct and return the path
            if current == goal {
                return Ok(self.reconstruct_path(goal));
            }

            // Move from open to closed set
            self.move_open_to_closed(&current);

            // Get all hyperedges containing the current vertex
            self.graph
                .find_edges_with_node(&current)
                .for_each(|edge_id| {
                    // Get all vertices in this hyperedge
                    let vertices = self
                        .graph
                        .get_edge_domain(edge_id)
                        .expect("Failed to get edge vertices");

                    // Process each vertex in this hyperedge
                    for &neighbor in vertices {
                        // Skip if this is the current vertex or already evaluated
                        if neighbor == current || self.has_visited(&neighbor) {
                            continue;
                        }

                        // Cost to reach neighbor through current vertex
                        let tentative_g_score = self.g_score[&current] + 1.0;

                        // Check if this path is better than any previous path
                        let is_better_path = !self.has_g_score(&neighbor)
                            || tentative_g_score < self.g_score[&neighbor];

                        if is_better_path {
                            // Update path info
                            self.came_from_mut().insert(neighbor, current);
                            self.g_score_mut().insert(neighbor, tentative_g_score);

                            // Update f_score (g_score + heuristic)
                            let f_score =
                                tentative_g_score + self.heuristic().compute(neighbor, goal);
                            self.f_score_mut().insert(neighbor, f_score);

                            // Add to open set if not already there
                            if !self.in_open_set(&neighbor) {
                                self.open_set_mut().insert(neighbor);
                            }

                            // push the neighbor into the priority queue with its f_score (negative for min-heap behavior)
                            priority_queue.push(PriorityNode {
                                vertex: neighbor,
                                priority: -(f_score as i64),
                            });
                        }
                    }
                });
        }

        // No path found
        Err(Error::PathNotFound)
    }

    // Reconstruct path from came_from map
    fn reconstruct_path(&self, goal: VertexId<A::Ix>) -> Self::Path {
        let mut path = vec![goal];
        let mut current = goal;

        while let Some(&prev) = self.came_from.get(&current) {
            path.push(prev);
            current = prev;
        }

        path.reverse();
        path
    }
}

impl<'a, N, E, F, A, H, S> Traversal<VertexId<A::Ix>> for AStarSearch<'a, N, E, A, F, H, S>
where
    A: GraphProps,
    F: Heuristic<A::Ix, Output = f64>,
    H: HyperGraph<N, E, A>,
    S: BuildHasher,
    A::Ix: Eq + Hash,
{
    type Store<U> = HashSet<U, S>;

    fn has_visited(&self, vertex: &VertexId<A::Ix>) -> bool {
        self.visited().contains(vertex)
    }

    fn visited(&self) -> &Self::Store<VertexId<A::Ix>> {
        self.closed_set()
    }
}

impl<'a, N, E, F, A, H, S> Search<VertexId<A::Ix>> for AStarSearch<'a, N, E, A, F, H, S>
where
    A: GraphProps,
    F: Heuristic<A::Ix, Output = f64>,
    H: HyperGraphIter<N, E, A>,
    S: BuildHasher,
    A::Ix: HyperIndex,
    for<'b> &'b <H::Edge<E> as RawLayout>::Store: IntoIterator<Item = &'b VertexId<A::Ix>>,
{
    type Output = Vec<VertexId<A::Ix>>;

    fn search(&mut self, start: VertexId<A::Ix>) -> Result<Self::Output> {
        // For A*, we need a goal vertex to compute the heuristic
        // This implementation of search will explore the graph and return
        // all reachable vertices ordered by their distance from start
        self.reset();

        if !self.graph.contains_node(&start) {
            return Err(rshyper::Error::NodeNotFound.into());
        }

        // Using the vertex with the largest ID as a pseudo-goal
        // This is a hack to make A* behave more like a general search
        let max_vertex_id = match self.graph.vertices().max() {
            Some(&id) => id,
            None => return Ok(vec![]),
        };

        self.find_path(start, max_vertex_id)
    }
}

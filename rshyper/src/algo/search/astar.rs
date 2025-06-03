/*
    Appellation: impl_astar <module>
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::priority_node::PriorityNode;

pub(crate) mod priority_node;

use super::{Search, Traversal};
use crate::hash_graph::{HashGraph, VertexSet};
use rshyper_core::index::{NumIndex, RawIndex, VertexId};
use rshyper_core::{GraphKind, HyperGraphAttributes};
use std::collections::{BinaryHeap, HashMap, HashSet};

/// A simple trait defining a common interface for heuristic functions compatible with the
/// [`A*`](AStarSearch) search implementation
pub trait HeuristicFunc<T = crate::Udx> {
    type Output;

    fn compute(&self, start: VertexId<T>, goal: VertexId<T>) -> Self::Output;
}

impl<F, Idx> HeuristicFunc<Idx> for F
where
    Idx: RawIndex,
    F: Fn(VertexId<Idx>, VertexId<Idx>) -> f64,
{
    type Output = f64;

    fn compute(&self, start: VertexId<Idx>, goal: VertexId<Idx>) -> Self::Output {
        self(start, goal)
    }
}

/// A* Search algorithm for hypergraphs
pub struct AStarSearch<'a, N, E, A, F>
where
    F: HeuristicFunc<A::Idx>,
    A: HyperGraphAttributes,
    A::Idx: Eq + core::hash::Hash,
{
    pub(crate) graph: &'a HashGraph<N, E, A>,
    pub(crate) open_set: VertexSet<A::Idx>,
    pub(crate) closed_set: VertexSet<A::Idx>,
    pub(crate) came_from: HashMap<VertexId<A::Idx>, VertexId<A::Idx>>,
    pub(crate) g_score: HashMap<VertexId<A::Idx>, F::Output>,
    pub(crate) f_score: HashMap<VertexId<A::Idx>, F::Output>,
    pub(crate) heuristic: F,
}

impl<'a, N, E, A, F, K, Idx> AStarSearch<'a, N, E, A, F>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    A: HyperGraphAttributes<Idx = Idx, Kind = K>,
    F: HeuristicFunc<Idx>,
    K: GraphKind,
    Idx: RawIndex + Eq + core::hash::Hash,
{
    /// Create a new A* search instance with the given heuristic function
    pub fn new(graph: &'a HashGraph<N, E, A>, heuristic: F) -> Self {
        Self {
            graph,
            open_set: VertexSet::new(),
            closed_set: VertexSet::new(),
            came_from: HashMap::new(),
            g_score: HashMap::new(),
            f_score: HashMap::new(),
            heuristic,
        }
    }
    /// consumes the current instance to create another from the given heuristic function;
    /// **note:** while the functions may be different, the output type of both must match.
    pub fn with_heuristic<G>(self, heuristic: G) -> AStarSearch<'a, N, E, A, G>
    where
        G: HeuristicFunc<Idx, Output = F::Output>,
    {
        AStarSearch {
            graph: self.graph,
            open_set: self.open_set,
            closed_set: self.closed_set,
            came_from: self.came_from,
            g_score: self.g_score,
            f_score: self.f_score,
            heuristic,
        }
    }

    pub const fn came_from(&self) -> &HashMap<VertexId<Idx>, VertexId<Idx>> {
        &self.came_from
    }
    /// returns a mutable reference to the map of vertices that have been processed
    pub const fn came_from_mut(&mut self) -> &mut HashMap<VertexId<Idx>, VertexId<Idx>> {
        &mut self.came_from
    }
    /// returns an immutable reference to the closed set of vertices
    pub const fn closed_set(&self) -> &VertexSet<Idx> {
        &self.closed_set
    }
    /// returns a mutable reference to the closed set of vertices
    pub const fn closed_set_mut(&mut self) -> &mut VertexSet<Idx> {
        &mut self.closed_set
    }
    /// returns an immutable reference to the f_score map
    pub const fn f_score(&self) -> &HashMap<VertexId<Idx>, F::Output> {
        &self.f_score
    }
    /// returns a mutable reference to the f_score map
    pub const fn f_score_mut(&mut self) -> &mut HashMap<VertexId<Idx>, F::Output> {
        &mut self.f_score
    }
    /// returns an immutable reference to the g_score map
    pub const fn g_score(&self) -> &HashMap<VertexId<Idx>, F::Output> {
        &self.g_score
    }
    /// returns a mutable reference to the g_score map
    pub const fn g_score_mut(&mut self) -> &mut HashMap<VertexId<Idx>, F::Output> {
        &mut self.g_score
    }
    /// returns an immutable reference to the heuristic function of the algorithm
    pub const fn heuristic(&self) -> &F {
        &self.heuristic
    }
    /// returns an immutable reference to the set of vertices that have been visited
    pub const fn open_set(&self) -> &VertexSet<Idx> {
        &self.open_set
    }
    /// returns amutable reference to the open set of vertices
    pub const fn open_set_mut(&mut self) -> &mut VertexSet<Idx> {
        &mut self.open_set
    }
    /// returns true if the given vertex has a f_score
    pub fn has_f_score<Q>(&self, vertex: &Q) -> bool
    where
        Q: Eq + core::hash::Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.f_score().contains_key(vertex)
    }
    /// returns true if the given vertex has a g_score
    pub fn has_g_score<Q>(&self, vertex: &Q) -> bool
    where
        Q: Eq + core::hash::Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.g_score().contains_key(vertex)
    }
    /// returns true if the given vertex has been visited
    pub fn has_visited<Q>(&self, vertex: &Q) -> bool
    where
        Q: Eq + core::hash::Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.closed_set().contains(vertex)
    }
    /// returns true if the given vertex is in the open set
    pub fn in_open_set<Q>(&self, vertex: &Q) -> bool
    where
        Q: Eq + core::hash::Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.open_set().contains(vertex)
    }
    /// moves the vertex from the open set before inserting it into the closed set; this is
    /// useful for updating the state, marking a node as processed.
    pub fn move_open_to_closed(&mut self, vertex: &VertexId<Idx>)
    where
        Idx: Copy,
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
    /// a convience method to perform a search
    pub fn search(
        &mut self,
        start: VertexId<Idx>,
    ) -> crate::Result<<Self as Search<VertexId<Idx>>>::Output>
    where
        Self: Search<VertexId<Idx>>,
    {
        Search::search(self, start)
    }
}

impl<'a, N, E, F, A, K, Idx> AStarSearch<'a, N, E, A, F>
where
    A: HyperGraphAttributes<Idx = Idx, Kind = K>,
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    F: HeuristicFunc<Idx, Output = f64>,
    K: GraphKind,
    Idx: NumIndex,
{
    /// Find the shortest path between start and goal vertices
    pub fn find_path(
        &mut self,
        start: VertexId<Idx>,
        goal: VertexId<Idx>,
    ) -> crate::Result<Vec<VertexId<Idx>>> {
        // Check if both vertices exist
        if !self.graph.contains_node(&start) {
            return Err(crate::Error::NodeNotFound);
        }
        if !self.graph.contains_node(&goal) {
            return Err(crate::Error::NodeNotFound);
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
            let edges = self.graph.get_edges_with_vertex(&current)?;

            edges.iter().for_each(|edge_id| {
                // Get all vertices in this hyperedge
                let vertices = self
                    .graph
                    .get_edge_vertices(edge_id)
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
                    let is_better_path =
                        !self.has_g_score(&neighbor) || tentative_g_score < self.g_score[&neighbor];

                    if is_better_path {
                        // Update path info
                        self.came_from_mut().insert(neighbor, current);
                        self.g_score_mut().insert(neighbor, tentative_g_score);

                        // Update f_score (g_score + heuristic)
                        let f_score = tentative_g_score + self.heuristic().compute(neighbor, goal);
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
        Err(crate::Error::PathNotFound)
    }

    // Reconstruct path from came_from map
    fn reconstruct_path(&self, goal: VertexId<Idx>) -> Vec<VertexId<Idx>>
    where
        Idx: NumIndex,
    {
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

impl<'a, N, E, F, A> Traversal<VertexId<A::Idx>> for AStarSearch<'a, N, E, A, F>
where
    A: HyperGraphAttributes,
    A::Idx: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    F: HeuristicFunc<A::Idx, Output = f64>,
{
    type Store<U> = HashSet<U>;

    fn has_visited(&self, vertex: &VertexId<A::Idx>) -> bool {
        self.visited().contains(vertex)
    }

    fn visited(&self) -> &Self::Store<VertexId<A::Idx>> {
        self.closed_set()
    }
}

impl<'a, N, E, F, A, K, Idx> Search<VertexId<Idx>> for AStarSearch<'a, N, E, A, F>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    A: HyperGraphAttributes<Idx = Idx, Kind = K>,
    F: HeuristicFunc<Idx, Output = f64>,
    K: GraphKind,
    Idx: NumIndex,
{
    type Output = Vec<VertexId<Idx>>;

    fn search(&mut self, start: VertexId<Idx>) -> crate::Result<Self::Output> {
        // For A*, we need a goal vertex to compute the heuristic
        // This implementation of search will explore the graph and return
        // all reachable vertices ordered by their distance from start
        self.reset();

        if !self.graph.contains_node(&start) {
            return Err(crate::Error::NodeNotFound);
        }

        // Using the vertex with the largest ID as a pseudo-goal
        // This is a hack to make A* behave more like a general search
        let max_vertex_id = match self.graph.nodes().keys().max() {
            Some(&id) => id,
            None => return Ok(vec![]),
        };

        self.find_path(start, max_vertex_id)
    }
}

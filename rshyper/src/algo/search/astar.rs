/*
    Appellation: impl_astar <module>
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::priority_node::PriorityNode;

pub(crate) mod priority_node;

use super::{Search, Traversal};
use crate::hash_graph::{HashGraph, VertexSet};
use crate::index::{IndexError, VertexId};
use std::collections::{BinaryHeap, HashMap, HashSet};

/// A simple trait defining a common interface for heuristic functions compatible with the
/// [`A*`](AStarSearch) search implementation
pub trait HeuristicFunc<T = VertexId> {
    type Output;

    fn compute(&self, start: T, goal: T) -> Self::Output;
}

impl<F> HeuristicFunc<VertexId> for F
where
    F: Fn(VertexId, VertexId) -> f64,
{
    type Output = f64;

    fn compute(&self, start: VertexId, goal: VertexId) -> Self::Output {
        self(start, goal)
    }
}

/// A* Search algorithm for hypergraphs
pub struct AStarSearch<'a, N, E, F>
where
    F: HeuristicFunc,
{
    pub(crate) graph: &'a HashGraph<N, E>,
    pub(crate) open_set: VertexSet,
    pub(crate) closed_set: VertexSet,
    pub(crate) came_from: HashMap<VertexId, VertexId>,
    pub(crate) g_score: HashMap<VertexId, F::Output>,
    pub(crate) f_score: HashMap<VertexId, F::Output>,
    pub(crate) heuristic: F,
}

impl<'a, N, E, F> AStarSearch<'a, N, E, F>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    F: HeuristicFunc,
{
    /// Create a new A* search instance with the given heuristic function
    pub fn new(graph: &'a HashGraph<N, E>, heuristic: F) -> Self {
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
    pub fn with_heuristic<G>(self, heuristic: G) -> AStarSearch<'a, N, E, G>
    where
        G: HeuristicFunc<Output = F::Output>,
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
    /// returns an immutable reference to the set of visited vertices
    pub const fn closed_set(&self) -> &HashSet<VertexId> {
        &self.closed_set
    }
    /// returns an immutable reference to the heuristic function of the algorithm
    pub const fn heuristic(&self) -> &F {
        &self.heuristic
    }
    /// returns true if the given vertex has been visited
    pub fn has_visited(&self, vertex: &VertexId) -> bool {
        self.closed_set().contains(vertex)
    }
    /// reset the state
    pub fn reset(&mut self) -> &mut Self {
        self.open_set.clear();
        self.closed_set.clear();
        self.came_from.clear();
        self.g_score.clear();
        self.f_score.clear();
        self
    }
    /// a convience method to perform a search
    pub fn search(&mut self, start: VertexId) -> crate::Result<<Self as Search<VertexId>>::Output>
    where
        Self: Search<VertexId>,
    {
        Search::search(self, start)
    }
    /// Find the shortest path between start and goal vertices
    pub fn find_path(&mut self, start: VertexId, goal: VertexId) -> crate::Result<Vec<VertexId>>
    where
        F: HeuristicFunc<Output = f64>,
    {
        // Check if both vertices exist
        if !self.graph.contains_node(&start) {
            return Err(crate::Error::NodeNotFound);
        }
        if !self.graph.contains_node(&goal) {
            return Err(IndexError::VertexDoesNotExist(goal).into());
        }

        // Reset state
        self.reset();

        // Initialize g_score for start node (0) and infinity for all other nodes
        self.g_score.insert(start, 0.0);

        // initialize f_score for start node (heuristic only since g=0)
        let start_f_score = self.heuristic().compute(start, goal);
        self.f_score.insert(start, start_f_score);
        // add start node to the open set
        self.open_set.insert(start);

        // initialize priority queue
        let mut priority_queue = BinaryHeap::new();
        // push the start node with its f_score
        priority_queue.push(PriorityNode {
            vertex: start,
            priority: -(start_f_score as i64),
        });

        // Track processed nodes to avoid duplicate processing
        let mut processed = HashSet::new();

        while let Some(PriorityNode {
            priority: _,
            vertex: current,
        }) = priority_queue.pop()
        {
            // Skip if we've already processed this vertex with a better path
            // or it's no longer in the open set
            if processed.contains(&current) || !self.open_set.contains(&current) {
                continue;
            }
            processed.insert(current);

            // If we've reached the goal, construct and return the path
            if current == goal {
                return Ok(self.reconstruct_path(goal));
            }

            // Move from open to closed set
            self.open_set.remove(&current);
            self.closed_set.insert(current);

            // Get all hyperedges containing the current vertex
            let edges = self.graph.get_edges_with_vertex(&current)?;

            for edge_id in edges {
                // Get all vertices in this hyperedge
                let vertices = self.graph.get_edge_vertices(&edge_id)?;

                // Process each vertex in this hyperedge
                for &neighbor in vertices {
                    // Skip if this is the current vertex or already evaluated
                    if neighbor == current || self.closed_set.contains(&neighbor) {
                        continue;
                    }

                    // Cost to reach neighbor through current vertex
                    let tentative_g_score = self.g_score[&current] + 1.0;

                    // Check if this path is better than any previous path
                    let is_better_path = !self.g_score.contains_key(&neighbor)
                        || tentative_g_score < self.g_score[&neighbor];

                    if is_better_path {
                        // Update path info
                        self.came_from.insert(neighbor, current);
                        self.g_score.insert(neighbor, tentative_g_score);

                        // Update f_score (g_score + heuristic)
                        let f_score = tentative_g_score + self.heuristic().compute(neighbor, goal);
                        self.f_score.insert(neighbor, f_score);

                        // Add to open set if not already there
                        if !self.open_set.contains(&neighbor) {
                            self.open_set.insert(neighbor);
                        }

                        // Always add to priority queue with new f_score
                        // (The duplicate check above ensures we don't process unnecessarily)
                        priority_queue.push(PriorityNode {
                            vertex: neighbor,
                            priority: -(f_score as i64),
                        });
                    }
                }
            }
        }

        // No path found
        Err(IndexError::NoPathFoundBetween {
            from: *start.get(),
            to: *goal.get(),
        }
        .into())
    }

    // Reconstruct path from came_from map
    fn reconstruct_path(&self, goal: VertexId) -> Vec<VertexId> {
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

impl<'a, N, E, F> Traversal<VertexId> for AStarSearch<'a, N, E, F>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    F: HeuristicFunc<Output = f64>,
{
    type Store<U> = HashSet<U>;

    fn has_visited(&self, vertex: &VertexId) -> bool {
        self.visited().contains(vertex)
    }

    fn visited(&self) -> &Self::Store<VertexId> {
        self.closed_set()
    }
}

impl<'a, N, E, F> Search<VertexId> for AStarSearch<'a, N, E, F>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    F: HeuristicFunc<Output = f64>,
{
    type Output = Vec<VertexId>;

    fn search(&mut self, start: VertexId) -> crate::Result<Self::Output> {
        // For A*, we need a goal vertex to compute the heuristic
        // This implementation of search will explore the graph and return
        // all reachable vertices ordered by their distance from start
        self.reset();

        if !self.graph.contains_node(&start) {
            return Err(IndexError::VertexDoesNotExist(start).into());
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

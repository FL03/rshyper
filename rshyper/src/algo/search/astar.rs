/*
    Appellation: impl_astar <module>
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::priority_node::PriorityNode;

pub(crate) mod priority_node;

use super::{Search, Traversal};
use crate::HashGraph;
use crate::{Error, Result, VertexId};
use std::collections::{BinaryHeap, HashMap, HashSet};

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

impl<'a, N, E, F> AStarSearch<'a, N, E, F>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    F: Fn(VertexId, VertexId) -> f64,
{
    /// Create a new A* search instance with the given heuristic function
    pub fn new(graph: &'a HashGraph<N, E>, heuristic: F) -> Self {
        Self {
            graph,
            open_set: HashSet::new(),
            closed_set: HashSet::new(),
            came_from: HashMap::new(),
            g_score: HashMap::new(),
            f_score: HashMap::new(),
            heuristic,
        }
    }
    /// a convience method to perform a search
    pub fn search(&mut self, start: VertexId) -> Result<Vec<VertexId>> {
        Search::search(self, start)
    }
    /// reset the state
    pub fn reset(&mut self) {
        self.open_set.clear();
        self.closed_set.clear();
        self.came_from.clear();
        self.g_score.clear();
        self.f_score.clear();
    }

    /// Find the shortest path between start and goal vertices
    pub fn find_path(&mut self, start: VertexId, goal: VertexId) -> Result<Vec<VertexId>> {
        // Check if both vertices exist
        if !self.graph.contains_node(&start) {
            return Err(Error::VertexDoesNotExist(start));
        }
        if !self.graph.contains_node(&goal) {
            return Err(Error::VertexDoesNotExist(goal));
        }

        // Reset state
        self.reset();

        // Initialize g_score for start node (0) and infinity for all other nodes
        self.g_score.insert(start, 0.0);

        // Initialize f_score for start node (heuristic only since g=0)
        let start_f_score = (self.heuristic)(start, goal);
        self.f_score.insert(start, start_f_score);

        // Add start node to the open set
        self.open_set.insert(start);

        // Create a priority queue and add the start node
        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(PriorityNode {
            vertex: start,
            priority: -(start_f_score as i64),
        });

        // Track processed nodes to avoid duplicate processing
        let mut processed = HashSet::new();

        while !priority_queue.is_empty() {
            // Get node with lowest f_score
            let current = match priority_queue.pop() {
                Some(node) => node.vertex,
                None => break, // Should never happen if priority queue is not empty
            };

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
            let edges = match self.graph.get_edges_with_vertex(current) {
                Ok(edges) => edges,
                Err(e) => return Err(e),
            };

            for edge_id in edges {
                // Get all vertices in this hyperedge
                let vertices = match self.graph.get_vertices_for_edge(edge_id) {
                    Ok(verts) => verts,
                    Err(e) => return Err(e),
                };

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
                        let f_score = tentative_g_score + (self.heuristic)(neighbor, goal);
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
        Err(Error::Unknown(format!(
            "No path found from {} to {}",
            start, goal
        )))
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

    pub fn has_visited(&self, vertex: &VertexId) -> bool {
        self.visited().contains(vertex)
    }

    pub const fn visited(&self) -> &HashSet<VertexId> {
        &self.closed_set
    }
}

impl<'a, N, E, F> Traversal<VertexId> for AStarSearch<'a, N, E, F>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    F: Fn(VertexId, VertexId) -> f64,
{
    fn visited(&self) -> &HashSet<VertexId> {
        &self.closed_set
    }
}

impl<'a, N, E, F> Search<VertexId> for AStarSearch<'a, N, E, F>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    F: Fn(VertexId, VertexId) -> f64,
{
    type Output = Vec<VertexId>;

    fn search(&mut self, start: VertexId) -> crate::Result<Self::Output> {
        // For A*, we need a goal vertex to compute the heuristic
        // This implementation of search will explore the graph and return
        // all reachable vertices ordered by their distance from start
        self.reset();

        if !self.graph.contains_node(&start) {
            return Err(Error::VertexDoesNotExist(start));
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

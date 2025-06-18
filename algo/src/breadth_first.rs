/*
    Appellation: bft <module>
    Contrib: @FL03
*/
//! this module implements the breadth-first search algorithm as an operator on the hypergraph.
use crate::error::{Error, Result};
use crate::{Search, Traversal};
use core::hash::Hash;
use rshyper::idx::{NumIndex, RawIndex, VertexId};
use rshyper::rel::RawLayout;
use rshyper::{GraphProps, GraphType, HyperGraph};
use std::collections::{HashSet, VecDeque};

/// Breadth-First Traversal algorithm for hypergraphs
pub struct BreadthFirstTraversal<'a, N, E, A, H>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
{
    pub(crate) graph: &'a H,
    pub(crate) queue: VecDeque<VertexId<A::Ix>>,
    pub(crate) visited: HashSet<VertexId<A::Ix>>,
    _marker: core::marker::PhantomData<(N, E)>,
}

impl<'a, N, E, A, H, K, Idx> BreadthFirstTraversal<'a, N, E, A, H>
where
    A: GraphProps<Ix = Idx, Kind = K>,
    H: HyperGraph<N, E, A>,
    K: GraphType,
    Idx: RawIndex,
{
    /// create a new instance from a hypergraph
    pub fn new(graph: &'a H) -> Self {
        Self {
            graph,
            queue: VecDeque::new(),
            visited: HashSet::new(),
            _marker: core::marker::PhantomData::<(N, E)>,
        }
    }
    /// returns an immutable reference to the queue
    pub const fn queue(&self) -> &VecDeque<VertexId<Idx>> {
        &self.queue
    }
    /// returns a mutable reference to the queue
    pub(crate) const fn queue_mut(&mut self) -> &mut VecDeque<VertexId<Idx>> {
        &mut self.queue
    }
    /// returns an immutable reference to the visited vertices
    pub const fn visited(&self) -> &HashSet<VertexId<Idx>> {
        &self.visited
    }
    /// returns a mutable reference to the visited vertices
    pub const fn visited_mut(&mut self) -> &mut HashSet<VertexId<Idx>> {
        &mut self.visited
    }
    /// returns true if the vertex has been visited
    pub fn has_visited<Q>(&self, vertex: &Q) -> bool
    where
        Idx: Eq + Hash,
        Q: ?Sized + Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.visited().contains(vertex)
    }
    /// Reset the traversal state to allow reusing the instance
    pub fn reset(&mut self) -> &mut Self {
        self.queue_mut().clear();
        self.visited_mut().clear();
        self
    }
    /// a convience method to perform a search
    pub fn search(&mut self, start: VertexId<Idx>) -> Result<Vec<VertexId<Idx>>>
    where
        Idx: NumIndex,
        for<'b> &'b <H::Edge<E> as RawLayout>::Store: IntoIterator<Item = &'b VertexId<Idx>>,
    {
        Search::search(self, start)
    }
    /// if the vertex hans't been visited yet, push it to the back of the queue and mark it as
    /// visited by inserting it into the visited set.
    pub(crate) fn register(&mut self, vertex: VertexId<Idx>)
    where
        Idx: Copy + Eq + Hash,
    {
        if !self.has_visited(&vertex) {
            self.queue_mut().push_back(vertex);
            self.visited_mut().insert(vertex);
        }
    }
}

impl<'a, N, E, A, H, K, Idx> Search<VertexId<Idx>> for BreadthFirstTraversal<'a, N, E, A, H>
where
    A: GraphProps<Ix = Idx, Kind = K>,
    H: HyperGraph<N, E, A>,
    K: GraphType,
    Idx: NumIndex,
    for<'b> &'b <H::Edge<E> as RawLayout>::Store: IntoIterator<Item = &'b VertexId<Idx>>,
{
    type Output = Vec<VertexId<Idx>>;

    fn search(&mut self, start: VertexId<Idx>) -> Result<Self::Output> {
        // Reset state
        self.reset();

        // Check if starting vertex exists
        if !self.graph.contains_node(&start) {
            return Err(Error::NotFound(Box::new(start.value())));
        }

        // Add start vertex to queue and mark as visited
        self.queue.push_back(start);
        self.visited.insert(start);

        // Path to return (traversal order)
        let mut path = Vec::new();

        // BFT algorithm
        while let Some(current) = self.queue_mut().pop_front() {
            path.push(current);

            // visit all vertices within each edge that haven't been visited yet
            for edge_id in self.graph.find_edges_with_node(&current) {
                for vertex in self.graph.get_edge_domain(&edge_id).expect("Edge is empty") {
                    self.register(*vertex);
                }
            }
        }

        Ok(path)
    }
}

impl<'a, N, E, A, H, K, Idx> Traversal<VertexId<Idx>> for BreadthFirstTraversal<'a, N, E, A, H>
where
    A: GraphProps<Ix = Idx, Kind = K>,
    H: HyperGraph<N, E, A>,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
{
    type Store<I2> = HashSet<I2>;

    fn has_visited(&self, vertex: &VertexId<Idx>) -> bool {
        self.visited().contains(vertex)
    }

    fn visited(&self) -> &Self::Store<VertexId<Idx>> {
        self.visited()
    }
}

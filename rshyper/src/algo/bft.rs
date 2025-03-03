/*
    Appellation: bft <module>
    Contrib: @FL03
*/

pub struct BreadthFirstTraversal<'a, G> {
    graph: &'a G,
    queue: Vec<usize>,
    visited: Vec<bool>,
}
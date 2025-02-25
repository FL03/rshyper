use std::collections::HashSet;

/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use rshyper::{HyperGraph, Index};

#[test]
fn test_hypergraph() {
    let mut graph = HyperGraph::new();

    // Add some vertices
    let v0 = graph.add_vertex();
    let v1 = graph.add_vertex();
    let v2 = graph.add_vertex();
    let v3 = graph.add_vertex();

    // Add some hyperedges
    assert!(graph.add_hyperedge(vec![v0, v1, v2]).is_ok());
    assert!(graph.add_hyperedge(vec![v1, v2, v3]).is_ok());

    // Get neighbors of vertex v1
    let exp = HashSet::<Index>::from_iter([Index(2), Index(3), Index(0)]);
    assert_eq!(graph.get_neighbors(v1).expect("neighbors"), exp);

    // Get degree of vertex v1
    assert_eq!(graph.vertex_degree(v1).expect("degree"), 2);

    // Remove a vertex
    assert!(graph.remove_vertex(v2).is_ok());
}

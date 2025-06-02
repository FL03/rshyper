/*
    Appellation: search <test>
    Contrib: @FL03
*/
use rshyper::hash_graph::UndirectedHashGraph as HyperGraph;

#[test]
fn test_breadth_first_traversal() {
    let mut graph = HyperGraph::<usize, usize>::undirected();

    // Create a simple hypergraph
    // 0 -- 1 -- 3
    //  \  /
    //   2 -- 4

    let v0 = graph.add_vertex();
    let v1 = graph.add_vertex();
    let v2 = graph.add_vertex();
    let v3 = graph.add_vertex();
    let v4 = graph.add_vertex();

    let _e1 = graph.add_edge(vec![v0, v1, v2]).unwrap();
    let _e2 = graph.add_edge(vec![v1, v3]).unwrap();
    let _e3 = graph.add_edge(vec![v2, v4]).unwrap();

    let mut bft = graph.bft();
    let path = bft.search(v0).unwrap();

    // Verify traversal order - should be breadth-first
    // (note: exact order might vary for vertices at the same level)
    assert_eq!(path.len(), 5, "Should visit all 5 vertices");
    assert_eq!(path[0], v0, "Should start with v0");

    // All vertices should be visited
    let visited = bft.visited();
    assert!(visited.contains(&v0));
    assert!(visited.contains(&v1));
    assert!(visited.contains(&v2));
    assert!(visited.contains(&v3));
    assert!(visited.contains(&v4));
}

#[test]
fn test_bft_cyclic_graph() {
    let mut graph = HyperGraph::<usize, usize>::undirected();

    // Create a cyclic hypergraph
    // 0 -- 1 -- 2
    // |         |
    // +----3----+

    let v0 = graph.add_vertex();
    let v1 = graph.add_vertex();
    let v2 = graph.add_vertex();
    let v3 = graph.add_vertex();

    let _e1 = graph.add_edge(vec![v0, v1]).unwrap();
    let _e2 = graph.add_edge(vec![v1, v2]).unwrap();
    let _e3 = graph.add_edge(vec![v2, v3]).unwrap();
    let _e4 = graph.add_edge(vec![v3, v0]).unwrap();

    let mut bft = graph.bft();
    let path = bft.search(v0).unwrap();

    // BFT should visit each vertex exactly once
    assert_eq!(path.len(), 4, "Should visit each vertex exactly once");

    // All vertices should be visited
    let visited = bft.visited();
    assert!(visited.contains(&v0));
    assert!(visited.contains(&v1));
    assert!(visited.contains(&v2));
    assert!(visited.contains(&v3));
}

#[test]
fn test_bft_disconnected_graph() {
    let mut graph = HyperGraph::<usize, usize>::undirected();

    // Create a disconnected hypergraph
    // 0 -- 1    2 -- 3

    let v0 = graph.add_vertex();
    let v1 = graph.add_vertex();
    let v2 = graph.add_vertex();
    let v3 = graph.add_vertex();

    let _e1 = graph.add_edge(vec![v0, v1]).unwrap();
    let _e2 = graph.add_edge(vec![v2, v3]).unwrap();

    let mut bft = graph.bft();
    let path = bft.search(v0).unwrap();

    // BFT should only visit connected vertices (v0 and v1)
    assert_eq!(path.len(), 2, "Should only visit connected vertices");
    assert!(path.contains(&v0));
    assert!(path.contains(&v1));

    // v2 and v3 should not be visited
    let visited = bft.visited();
    assert!(!visited.contains(&v2));
    assert!(!visited.contains(&v3));
}

#[test]
fn test_bft_hyperedge_with_multiple_vertices() {
    let mut graph = HyperGraph::<usize, usize>::undirected();

    // Create a hypergraph with a large hyperedge
    // connecting multiple vertices
    let v0 = graph.add_vertex();
    let v1 = graph.add_vertex();
    let v2 = graph.add_vertex();
    let v3 = graph.add_vertex();
    let v4 = graph.add_vertex();

    // Single hyperedge connecting all vertices
    let _e1 = graph.add_edge(vec![v0, v1, v2, v3, v4]).unwrap();

    let mut bft = graph.bft();
    let path = bft.search(v0).unwrap();

    // All vertices should be in the path and at the same level
    assert_eq!(path.len(), 5, "All vertices should be visited");
    assert_eq!(path[0], v0, "Should start with v0");

    // The remaining vertices should all be at the same level
    // (order might vary but they should all be visited)
    let visited = bft.visited();
    assert!(visited.contains(&v0));
    assert!(visited.contains(&v1));
    assert!(visited.contains(&v2));
    assert!(visited.contains(&v3));
    assert!(visited.contains(&v4));
}

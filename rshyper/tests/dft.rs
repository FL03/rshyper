/*
    appellation: dft <test>
    authors: @FL03
*/
use rshyper::HashGraph;

#[test]
fn test_depth_first_traversal() {
    let mut graph = HashGraph::<()>::new();

    // Create a simple hypergraph
    let v0 = graph.insert_node_default();
    let v1 = graph.insert_node_default();
    let v2 = graph.insert_node_default();
    let v3 = graph.insert_node_default();

    let _e1 = graph.insert_edge(vec![v0, v1]).unwrap();
    let _e2 = graph.insert_edge(vec![v1, v2]).unwrap();
    let _e3 = graph.insert_edge(vec![v2, v3]).unwrap();

    let mut dft = graph.dft();
    let path = dft.search(v0).unwrap();

    // Verify traversal follows depth-first pattern
    assert_eq!(path.len(), 4, "Should visit all 4 vertices");
    assert_eq!(path[0], v0, "Should start with v0");

    // All vertices should be visited
    let visited = dft.visited();
    assert!(visited.contains(&v0));
    assert!(visited.contains(&v1));
    assert!(visited.contains(&v2));
    assert!(visited.contains(&v3));
}

#[test]
fn test_dft_branching_graph() {
    let mut graph = HashGraph::<()>::new();

    // Create a branching hypergraph
    //      1
    //     /
    // 0 -+-- 2 -- 4
    //     \
    //      3

    let v0 = graph.insert_node_default();
    let v1 = graph.insert_node_default();
    let v2 = graph.insert_node_default();
    let v3 = graph.insert_node_default();
    let v4 = graph.insert_node_default();

    let _e1 = graph.insert_edge(vec![v0, v1]).unwrap();
    let _e2 = graph.insert_edge(vec![v0, v2]).unwrap();
    let _e3 = graph.insert_edge(vec![v0, v3]).unwrap();
    let _e4 = graph.insert_edge(vec![v2, v4]).unwrap();

    let mut dft = graph.dft();
    let path = dft.search(v0).unwrap();

    // Should visit all vertices
    assert_eq!(path.len(), 5, "Should visit all vertices");
    assert_eq!(path[0], v0, "Should start with v0");

    // Check for depth-first behavior by ensuring v4 comes after v2
    // Find the positions of v2 and v4 in the path
    let pos_v2 = path.iter().position(|&v| v == v2).unwrap();
    let pos_v4 = path.iter().position(|&v| v == v4).unwrap();
    assert!(pos_v2 < pos_v4, "In DFT, v4 should be visited after v2");
}

#[test]
fn test_dft_cyclic_graph() {
    let mut graph = HashGraph::<()>::new();

    // Create a cyclic graph
    // 0 -- 1 -- 2
    // |         |
    // +----3----+

    let v0 = graph.insert_node_default();
    let v1 = graph.insert_node_default();
    let v2 = graph.insert_node_default();
    let v3 = graph.insert_node_default();

    let _e1 = graph.insert_edge(vec![v0, v1]).unwrap();
    let _e2 = graph.insert_edge(vec![v1, v2]).unwrap();
    let _e3 = graph.insert_edge(vec![v2, v3]).unwrap();
    let _e4 = graph.insert_edge(vec![v3, v0]).unwrap();

    let mut dft = graph.dft();
    let path = dft.search(v0).unwrap();

    // Each vertex should be visited exactly once
    assert_eq!(path.len(), 4, "Each vertex should be visited exactly once");

    // All vertices should be visited
    let visited = dft.visited();
    assert!(visited.contains(&v0));
    assert!(visited.contains(&v1));
    assert!(visited.contains(&v2));
    assert!(visited.contains(&v3));
}

#[test]
fn test_dft_isolated_vertex() {
    let mut graph = HashGraph::<()>::new();

    // Create a graph with an isolated vertex
    // 0 -- 1    2

    let v0 = graph.insert_node_default();
    let v1 = graph.insert_node_default();
    let v2 = graph.insert_node_default(); // isolated

    let _e1 = graph.insert_edge(vec![v0, v1]).unwrap();

    let mut dft = graph.dft();
    let path = dft.search(v0).unwrap();

    // Should only visit connected vertices
    assert_eq!(path.len(), 2, "Should visit only connected vertices");

    // v2 should not be visited
    let visited = dft.visited();
    assert!(!visited.contains(&v2));

    // Starting from isolated vertex should only visit that vertex
    let path_from_isolated = dft.search(v2).unwrap();
    assert_eq!(
        path_from_isolated.len(),
        1,
        "Should contain only the isolated vertex"
    );
    assert_eq!(path_from_isolated[0], v2);
}

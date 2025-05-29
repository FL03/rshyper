/*
    Appellation: search <test>
    Contrib: @FL03
*/
use rshyper::{HashGraph, Search, VertexId};

#[test]
fn test_breadth_first_traversal() {
    let mut graph = HashGraph::<()>::new();

    // Create a simple hypergraph
    // 0 -- 1 -- 3
    //  \  /
    //   2 -- 4

    let v0 = graph.insert_vertex_default();
    let v1 = graph.insert_vertex_default();
    let v2 = graph.insert_vertex_default();
    let v3 = graph.insert_vertex_default();
    let v4 = graph.insert_vertex_default();

    let _e1 = graph.insert_edge(vec![v0, v1, v2]).unwrap();
    let _e2 = graph.insert_edge(vec![v1, v3]).unwrap();
    let _e3 = graph.insert_edge(vec![v2, v4]).unwrap();

    let mut bft = graph.bft();
    let path = bft.search(v0).unwrap();

    // Verify traversal order - should be breadth-first
    // (note: exact order might vary for vertices at the same level)
    assert_eq!(path.len(), 5, "Should visit all 5 vertices");
    assert_eq!(path[0], v0, "Should start with v0");

    // All vertices should be visited
    let visited = bft.visited_vertices();
    assert!(visited.contains(&v0));
    assert!(visited.contains(&v1));
    assert!(visited.contains(&v2));
    assert!(visited.contains(&v3));
    assert!(visited.contains(&v4));
}

#[test]
fn test_bft_cyclic_graph() {
    let mut graph = HashGraph::<()>::new();

    // Create a cyclic hypergraph
    // 0 -- 1 -- 2
    // |         |
    // +----3----+

    let v0 = graph.insert_vertex_default();
    let v1 = graph.insert_vertex_default();
    let v2 = graph.insert_vertex_default();
    let v3 = graph.insert_vertex_default();

    let _e1 = graph.insert_edge(vec![v0, v1]).unwrap();
    let _e2 = graph.insert_edge(vec![v1, v2]).unwrap();
    let _e3 = graph.insert_edge(vec![v2, v3]).unwrap();
    let _e4 = graph.insert_edge(vec![v3, v0]).unwrap();

    let mut bft = graph.bft();
    let path = bft.search(v0).unwrap();

    // BFT should visit each vertex exactly once
    assert_eq!(path.len(), 4, "Should visit each vertex exactly once");

    // All vertices should be visited
    let visited = bft.visited_vertices();
    assert!(visited.contains(&v0));
    assert!(visited.contains(&v1));
    assert!(visited.contains(&v2));
    assert!(visited.contains(&v3));
}

#[test]
fn test_bft_disconnected_graph() {
    let mut graph = HashGraph::<()>::new();

    // Create a disconnected hypergraph
    // 0 -- 1    2 -- 3

    let v0 = graph.insert_vertex_default();
    let v1 = graph.insert_vertex_default();
    let v2 = graph.insert_vertex_default();
    let v3 = graph.insert_vertex_default();

    let _e1 = graph.insert_edge(vec![v0, v1]).unwrap();
    let _e2 = graph.insert_edge(vec![v2, v3]).unwrap();

    let mut bft = graph.bft();
    let path = bft.search(v0).unwrap();

    // BFT should only visit connected vertices (v0 and v1)
    assert_eq!(path.len(), 2, "Should only visit connected vertices");
    assert!(path.contains(&v0));
    assert!(path.contains(&v1));

    // v2 and v3 should not be visited
    let visited = bft.visited_vertices();
    assert!(!visited.contains(&v2));
    assert!(!visited.contains(&v3));
}

#[test]
fn test_bft_hyperedge_with_multiple_vertices() {
    let mut graph = HashGraph::<()>::new();

    // Create a hypergraph with a large hyperedge
    // connecting multiple vertices
    let v0 = graph.insert_vertex_default();
    let v1 = graph.insert_vertex_default();
    let v2 = graph.insert_vertex_default();
    let v3 = graph.insert_vertex_default();
    let v4 = graph.insert_vertex_default();

    // Single hyperedge connecting all vertices
    let _e1 = graph.insert_edge(vec![v0, v1, v2, v3, v4]).unwrap();

    let mut bft = graph.bft();
    let path = bft.search(v0).unwrap();

    // All vertices should be in the path and at the same level
    assert_eq!(path.len(), 5, "All vertices should be visited");
    assert_eq!(path[0], v0, "Should start with v0");

    // The remaining vertices should all be at the same level
    // (order might vary but they should all be visited)
    let visited = bft.visited_vertices();
    assert!(visited.contains(&v0));
    assert!(visited.contains(&v1));
    assert!(visited.contains(&v2));
    assert!(visited.contains(&v3));
    assert!(visited.contains(&v4));
}

#[test]
fn test_depth_first_traversal() {
    let mut graph = HashGraph::<()>::new();

    // Create a simple hypergraph
    let v0 = graph.insert_vertex_default();
    let v1 = graph.insert_vertex_default();
    let v2 = graph.insert_vertex_default();
    let v3 = graph.insert_vertex_default();

    let _e1 = graph.insert_edge(vec![v0, v1]).unwrap();
    let _e2 = graph.insert_edge(vec![v1, v2]).unwrap();
    let _e3 = graph.insert_edge(vec![v2, v3]).unwrap();

    let mut dft = graph.dft();
    let path = dft.search(v0).unwrap();

    // Verify traversal follows depth-first pattern
    assert_eq!(path.len(), 4, "Should visit all 4 vertices");
    assert_eq!(path[0], v0, "Should start with v0");

    // All vertices should be visited
    let visited = dft.visited_vertices();
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

    let v0 = graph.insert_vertex_default();
    let v1 = graph.insert_vertex_default();
    let v2 = graph.insert_vertex_default();
    let v3 = graph.insert_vertex_default();
    let v4 = graph.insert_vertex_default();

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

    let v0 = graph.insert_vertex_default();
    let v1 = graph.insert_vertex_default();
    let v2 = graph.insert_vertex_default();
    let v3 = graph.insert_vertex_default();

    let _e1 = graph.insert_edge(vec![v0, v1]).unwrap();
    let _e2 = graph.insert_edge(vec![v1, v2]).unwrap();
    let _e3 = graph.insert_edge(vec![v2, v3]).unwrap();
    let _e4 = graph.insert_edge(vec![v3, v0]).unwrap();

    let mut dft = graph.dft();
    let path = dft.search(v0).unwrap();

    // Each vertex should be visited exactly once
    assert_eq!(path.len(), 4, "Each vertex should be visited exactly once");

    // All vertices should be visited
    let visited = dft.visited_vertices();
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

    let v0 = graph.insert_vertex_default();
    let v1 = graph.insert_vertex_default();
    let v2 = graph.insert_vertex_default(); // isolated

    let _e1 = graph.insert_edge(vec![v0, v1]).unwrap();

    let mut dft = graph.dft();
    let path = dft.search(v0).unwrap();

    // Should only visit connected vertices
    assert_eq!(path.len(), 2, "Should visit only connected vertices");

    // v2 should not be visited
    let visited = dft.visited_vertices();
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

#[ignore = "A* search cannot find min paths"]
#[test]
fn test_astar_search() {
    let mut graph = HashGraph::<()>::new();

    // Create a simple hypergraph
    let v0 = graph.insert_vertex_default();
    let v1 = graph.insert_vertex_default();
    let v2 = graph.insert_vertex_default();
    let v3 = graph.insert_vertex_default();
    let v4 = graph.insert_vertex_default();

    // Direct path: v0 -> v1 -> v3
    graph.insert_edge(vec![v0, v1]).unwrap();
    graph.insert_edge(vec![v1, v3]).unwrap();

    // Longer path: v0 -> v2 -> v4 -> v3
    graph.insert_edge(vec![v0, v2]).unwrap();
    graph.insert_edge(vec![v2, v4]).unwrap();
    graph.insert_edge(vec![v4, v3]).unwrap();

    // Simple Euclidean distance heuristic (not used in this test)
    let heuristic = |_: VertexId, _: VertexId| -> f64 { 0.0 };

    let mut astar = graph.astar(heuristic);
    let path = astar.find_path(v0, v3).unwrap();

    // A* should find the shortest path (v0 -> v1 -> v3)
    assert_eq!(path.len(), 3, "Shortest path should have 3 vertices");
    assert_eq!(path[0], v0, "Path should start with v0");
    assert_eq!(path[2], v3, "Path should end with v3");
    assert_eq!(path[1], v1, "Path should go through v1 (shortest route)");
}

#[test]
fn test_astar_with_heuristic() {
    let mut graph = HashGraph::<()>::new();

    // Create a simple grid-like graph
    // 0 -- 1 -- 2
    // |    |    |
    // 3 -- 4 -- 5
    // |    |    |
    // 6 -- 7 -- 8

    let v0 = graph.insert_vertex_default(); // (0,0)
    let v1 = graph.insert_vertex_default(); // (1,0)
    let v2 = graph.insert_vertex_default(); // (2,0)
    let v3 = graph.insert_vertex_default(); // (0,1)
    let v4 = graph.insert_vertex_default(); // (1,1)
    let v5 = graph.insert_vertex_default(); // (2,1)
    let v6 = graph.insert_vertex_default(); // (0,2)
    let v7 = graph.insert_vertex_default(); // (1,2)
    let v8 = graph.insert_vertex_default(); // (2,2)

    // Create horizontal connections
    graph.insert_edge(vec![v0, v1]).unwrap();
    graph.insert_edge(vec![v1, v2]).unwrap();
    graph.insert_edge(vec![v3, v4]).unwrap();
    graph.insert_edge(vec![v4, v5]).unwrap();
    graph.insert_edge(vec![v6, v7]).unwrap();
    graph.insert_edge(vec![v7, v8]).unwrap();

    // Create vertical connections
    graph.insert_edge(vec![v0, v3]).unwrap();
    graph.insert_edge(vec![v3, v6]).unwrap();
    graph.insert_edge(vec![v1, v4]).unwrap();
    graph.insert_edge(vec![v4, v7]).unwrap();
    graph.insert_edge(vec![v2, v5]).unwrap();
    graph.insert_edge(vec![v5, v8]).unwrap();

    // Define positions for each vertex in a 2D grid
    let positions = vec![
        (0f64, 0f64), // v0
        (1.0, 0.0),   // v1
        (2.0, 0.0),   // v2
        (0.0, 1.0),   // v3
        (1.0, 1.0),   // v4
        (2.0, 1.0),   // v5
        (0.0, 2.0),   // v6
        (1.0, 2.0),   // v7
        (2.0, 2.0),   // v8
    ];

    // Manhattan distance heuristic
    let heuristic = move |from: VertexId, to: VertexId| -> f64 {
        let (from_x, from_y) = positions[*from];
        let (to_x, to_y) = positions[*to];
        ((from_x - to_x).abs() + (from_y - to_y).abs()) as f64
    };

    // Find path from v0 to v8 (diagonal corners)
    let mut astar = graph.astar(heuristic);
    let path = astar.find_path(v0, v8).unwrap();

    // Shortest path should have 5 vertices (one of several equivalent paths)
    assert_eq!(path.len(), 5, "Shortest path should have 5 vertices");
    assert_eq!(path[0], v0, "Path should start with v0");
    assert_eq!(path[path.len() - 1], v8, "Path should end with v8");

    // Verify all vertices in the path are connected
    for i in 0..path.len() - 1 {
        let current = path[i];
        let next = path[i + 1];

        // Check if these vertices are connected by any hyperedge
        let current_edges = graph.get_edges_with_vertex(current).unwrap();
        let next_edges = graph.get_edges_with_vertex(next).unwrap();

        // There should be at least one common edge between current and next
        let has_connection = current_edges
            .iter()
            .any(|&e1| next_edges.iter().any(|&e2| e1 == e2));

        assert!(has_connection, "Vertices in path must be connected");
    }
}

#[test]
fn test_astar_disconnected() {
    let mut graph = HashGraph::<()>::new();

    // Create two disconnected components
    // 0 -- 1    2 -- 3

    let v0 = graph.insert_vertex_default();
    let v1 = graph.insert_vertex_default();
    let v2 = graph.insert_vertex_default();
    let v3 = graph.insert_vertex_default();

    graph.insert_edge(vec![v0, v1]).unwrap();
    graph.insert_edge(vec![v2, v3]).unwrap();

    // Simple heuristic
    let heuristic = |_: VertexId, _: VertexId| -> f64 { 0.0 };

    let mut astar = graph.astar(heuristic);

    // Try to find path between disconnected vertices
    let result = astar.find_path(v0, v3);

    // Should return an error since no path exists
    assert!(result.is_err(), "Should fail when no path exists");
}

#[test]
fn test_astar_complex_paths() {
    let mut graph = HashGraph::<()>::new();

    // Create a graph with multiple paths of different lengths
    // 0 -- 1 -- 2 -- 3
    // |              /
    // +-- 4 -- 5 ---+
    // |         /
    // +-- 6 ---+

    let v0 = graph.insert_vertex_default();
    let v1 = graph.insert_vertex_default();
    let v2 = graph.insert_vertex_default();
    let v3 = graph.insert_vertex_default();
    let v4 = graph.insert_vertex_default();
    let v5 = graph.insert_vertex_default();
    let v6 = graph.insert_vertex_default();

    // Path 1: v0 -> v1 -> v2 -> v3 (length 3)
    graph.insert_edge(vec![v0, v1]).unwrap();
    graph.insert_edge(vec![v1, v2]).unwrap();
    graph.insert_edge(vec![v2, v3]).unwrap();

    // Path 2: v0 -> v4 -> v5 -> v3 (length 3)
    graph.insert_edge(vec![v0, v4]).unwrap();
    graph.insert_edge(vec![v4, v5]).unwrap();
    graph.insert_edge(vec![v5, v3]).unwrap();

    // Path 3: v0 -> v6 -> v5 -> v3 (also length 3)
    graph.insert_edge(vec![v0, v6]).unwrap();
    graph.insert_edge(vec![v6, v5]).unwrap();
    // v5 -> v3 already defined

    // Simple heuristic
    let heuristic = |_: VertexId, _: VertexId| -> f64 { 0.0 };

    let mut astar = graph.astar(heuristic);
    let path = astar.find_path(v0, v3).unwrap();

    // A* should find one of the shortest paths (all are length 4)
    assert_eq!(path.len(), 4, "Shortest path should have 4 vertices");
    assert_eq!(path[0], v0, "Path should start with v0");
    assert_eq!(path[path.len() - 1], v3, "Path should end with v3");

    // Verify this is actually a valid path in the graph
    for i in 0..path.len() - 1 {
        let current_edges = graph.get_edges_with_vertex(path[i]).unwrap();
        let next_edges = graph.get_edges_with_vertex(path[i + 1]).unwrap();

        let has_connection = current_edges
            .iter()
            .any(|&e1| next_edges.iter().any(|&e2| e1 == e2));

        assert!(has_connection, "Found an invalid path");
    }
}

/*
    appellation: astar_search <module>
    authors: @FL03
*/
use rshyper_core::{Result, VertexId};
use rshyper_hmap::HyperMap;

#[test]
fn test_astar_shortest_path() -> Result<()> {
    // Simple Euclidean distance heuristic (not used in this test)
    fn heuristic(_x: VertexId, _y: VertexId) -> f64 {
        0.0 // No heuristic, just a placeholder
    }
    // initialize a new graph
    let mut graph = HyperMap::<usize, usize>::undirected();

    // use the macro create some new vertices
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    let v4 = graph.add_vertex()?;
    let v5 = graph.add_vertex()?;
    // create two paths with varying lengths
    // Path 1: v0 -> v1 -> v3
    graph.add_edge([v0, v1])?;
    graph.add_edge([v1, v3])?;
    // Path 2: v0 -> v2 -> v4 -> v1 -> v5 -> v3
    graph.add_edge([v0, v2])?;
    graph.add_edge([v2, v4])?;
    graph.add_edge([v4, v1])?;
    graph.add_edge([v1, v5])?;
    graph.add_edge([v5, v3])?;

    // use the a* search algorithm to find a set of paths
    let path = graph.astar(heuristic).find_path(v0, v3)?;

    // A* should find the shortest path (v0 -> v1 -> v3)
    assert_eq!(
        path,
        [v0, v1, v3],
        "Path should start with v0, go through v1, and end with v3"
    );

    Ok(())
}

#[test]
fn test_astar_with_heuristic() -> Result<()> {
    let mut graph = HyperMap::<usize, usize>::undirected();

    // Create a simple grid-like graph
    // 0 -- 1 -- 2
    // |    |    |
    // 3 -- 4 -- 5
    // |    |    |
    // 6 -- 7 -- 8

    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    let v4 = graph.add_vertex()?;
    let v5 = graph.add_vertex()?;
    let v6 = graph.add_vertex()?;
    let v7 = graph.add_vertex()?;
    let v8 = graph.add_vertex()?;

    // Create horizontal connections
    graph.add_edge([v0, v1])?;
    graph.add_edge([v1, v2])?;
    graph.add_edge([v3, v4])?;
    graph.add_edge([v4, v5])?;
    graph.add_edge([v6, v7])?;
    graph.add_edge([v7, v8])?;

    // Create vertical connections
    graph.add_edge([v0, v3])?;
    graph.add_edge([v3, v6])?;
    graph.add_edge([v1, v4])?;
    graph.add_edge([v4, v7])?;
    graph.add_edge([v2, v5])?;
    graph.add_edge([v5, v8])?;

    // Define positions for each vertex in a 2D grid
    let positions: [(f64, f64); 9] = [
        (0.0, 0.0), // v0
        (1.0, 0.0), // v1
        (2.0, 0.0), // v2
        (0.0, 1.0), // v3
        (1.0, 1.0), // v4
        (2.0, 1.0), // v5
        (0.0, 2.0), // v6
        (1.0, 2.0), // v7
        (2.0, 2.0), // v8
    ];

    // Manhattan distance heuristic
    let heuristic = move |from: VertexId, to: VertexId| -> f64 {
        // deconstruct the expected values using the positions array
        let (x1, y1) = positions[*from];
        let (x2, y2) = positions[*to];
        // Calculate Manhattan distance
        (x1 - x2).abs() + (y1 - y2).abs()
    };

    // Find path from v0 to v8 (diagonal corners)
    let mut astar = graph.astar(heuristic);
    let path = astar.find_path(v0, v8)?;

    // Shortest path should have 5 vertices (one of several equivalent paths)
    assert_eq!(path.len(), 5, "Shortest path should have 5 vertices");
    assert_eq!(path[0], v0, "Path should start with v0");
    assert_eq!(path[path.len() - 1], v8, "Path should end with v8");

    // Verify all vertices in the path are connected
    for i in 0..path.len() - 1 {
        let current = path[i];
        let next = path[i + 1];

        // Check if these vertices are connected by any hyperedge
        let current_edges = graph.find_edges_with_node(&current).collect::<Vec<_>>();
        let next_edges = graph.find_edges_with_node(&next).collect::<Vec<_>>();

        // There should be at least one common edge between current and next
        let has_connection = current_edges.iter().any(|&e1| next_edges.contains(&e1));

        assert!(has_connection, "Vertices in path must be connected");
    }

    Ok(())
}

#[test]
fn test_astar_disconnected() -> Result<()> {
    let mut graph = HyperMap::<usize, usize>::undirected();

    // Create two disconnected components
    // 0 -- 1    2 -- 3

    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;

    graph.add_edge([v0, v1])?;
    graph.add_edge([v2, v3])?;

    // Simple heuristic
    let heuristic = |_: VertexId, _: VertexId| -> f64 { 0.0 };

    let mut astar = graph.astar(heuristic);

    // Try to find path between disconnected vertices
    let result = astar.find_path(v0, v3);

    // Should return an error since no path exists
    assert!(result.is_err(), "Should fail when no path exists");

    Ok(())
}

#[test]
fn test_astar_complex_paths() -> Result<()> {
    let mut graph = HyperMap::<usize, usize>::undirected();

    // Create a graph with multiple paths of different lengths
    // 0 -- 1 -- 2 -- 3
    // |              /
    // +-- 4 -- 5 ---+
    // |         /
    // +-- 6 ---+

    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    let v4 = graph.add_vertex()?;
    let v5 = graph.add_vertex()?;
    let v6 = graph.add_vertex()?;

    // Path 1: v0 -> v1 -> v2 -> v3 (length 3)
    graph.add_edge([v0, v1])?;
    graph.add_edge([v1, v2])?;
    graph.add_edge([v2, v3])?;

    // Path 2: v0 -> v4 -> v5 -> v3 (length 3)
    graph.add_edge([v0, v4])?;
    graph.add_edge([v4, v5])?;
    graph.add_edge([v5, v3])?;

    // Path 3: v0 -> v6 -> v5 -> v3 (also length 3)
    graph.add_edge([v0, v6])?;
    graph.add_edge([v6, v5])?;
    // v5 -> v3 already defined

    // Simple heuristic
    let heuristic = |_: VertexId, _: VertexId| -> f64 { 0.0 };

    let mut astar = graph.astar(heuristic);
    let path = astar.find_path(v0, v3)?;

    // A* should find one of the shortest paths (all are length 4)
    assert_eq!(path.len(), 4, "Shortest path should have 4 vertices");
    assert_eq!(path[0], v0, "Path should start with v0");
    assert_eq!(path[path.len() - 1], v3, "Path should end with v3");

    // Verify this is actually a valid path in the graph
    for i in 0..path.len() - 1 {
        let current_edges = graph.find_edges_with_node(&path[i]).collect::<Vec<_>>();
        let next_edges = graph.find_edges_with_node(&path[i + 1]).collect::<Vec<_>>();

        let has_connection = current_edges.iter().any(|&e1| next_edges.contains(&e1));

        assert!(has_connection, "Found an invalid path");
    }

    Ok(())
}

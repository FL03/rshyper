/*
    appellation: dijkstra <module>
    authors: @FL03
*/
use rshyper::HashGraph;

#[test]
fn test_dijkstra_shortest_path() -> rshyper::Result<()> {
    // Initialize a new graph
    let mut graph = HashGraph::<usize, usize>::undirected();

    // Use the macro to create some new vertices
    rshyper::hypergraph! {
        graph {
            nodes: {
                let v0;
                let v1;
                let v2;
                let v3;
                let v4;
                let v5;
            };
            edges: {
                let _e0: [v0, v1]; // v0 -> v1
                let _e1: [v1, v2]; // v1 -> v2
                let _e2: [v2, v5]; // v2 -> v5
                let _e3: [v5, v4]; // v5 -> v4
                let _e4: [v0, v4]; // v0 -> v4
                let _e5: [v4, v3]; // v4 -> v3
            };
        }
    }

    // Use Dijkstra's algorithm to find the shortest path from v0 to v3
    let mut dijkstra = graph.dijkstra();
    // compute the shortest path from v0 to v3
    let path = dijkstra.find_path(v0, v3)?;

    // Dijkstra should find the shortest path (v0 -> v4 -> v3)
    assert_eq!(
        path,
        [v0, v4, v3],
        "Path should start with v0, go through v4 and end with v3"
    );

    Ok(())
}

#[test]
fn test_dijkstra_no_path() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize, usize>::undirected();

    rshyper::hypergraph! {
        graph {
            nodes: {
                let v0;
                let v1;
                let _v2;
                let v3;
            };
            edges: {
                let _e0: [v0, v1];
                // v2 and v3 are disconnected
            };
        }
    }

    let mut dijkstra = graph.dijkstra();
    let path = dijkstra.find_path(v0, v3);

    assert!(
        path.is_err() || path.as_ref().unwrap().is_empty(),
        "Should return error or empty path when no path exists"
    );

    Ok(())
}

#[test]
fn test_dijkstra_same_start_end() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize, usize>::undirected();

    rshyper::hypergraph! {
        graph {
            nodes: {
                let v0;
                let v1;
            };
            edges: {
                let _e0: [v0, v1];
            };
        }
    }

    let mut dijkstra = graph.dijkstra();
    let path = dijkstra.find_path(v0, v0)?;

    assert_eq!(
        path,
        [v0],
        "Path from a node to itself should be just that node"
    );

    Ok(())
}

#[test]
fn test_dijkstra_multiple_paths() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize, usize>::undirected();

    rshyper::hypergraph! {
        graph {
            nodes: {
                let v0;
                let v1;
                let v2;
                let v3;
            };
            edges: {
                let _e0: [v0, v1]; // v0-v1
                let _e1: [v1, v3]; // v1-v3
                let _e2: [v0, v2]; // v0-v2
                let _e3: [v2, v3]; // v2-v3
            };
        }
    }

    let mut dijkstra = graph.dijkstra();
    let path = dijkstra.find_path(v0, v3)?;

    // Both [v0, v1, v3] and [v0, v2, v3] are valid shortest paths
    assert!(
        path == [v0, v1, v3] || path == [v0, v2, v3],
        "Path should be one of the two shortest paths"
    );

    Ok(())
}

#[test]
fn test_dijkstra_direct_edge() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize, usize>::undirected();

    rshyper::hypergraph! {
        graph {
            nodes: {
                let v0;
                let v1;
            };
            edges: {
                let _e0: [v0, v1];
            };
        }
    }

    let mut dijkstra = graph.dijkstra();
    let path = dijkstra.find_path(v0, v1)?;

    assert_eq!(path, [v0, v1], "Direct edge should return direct path");

    Ok(())
}

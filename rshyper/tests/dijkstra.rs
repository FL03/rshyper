/*
    appellation: dijkstra <module>
    authors: @FL03
*/
#![allow(unused_variables)]
use rshyper::HyperMap;

#[test]
fn test_dijkstra_direct_edge() -> rshyper::Result<()> {
    // initializea new undirected hashgraph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // add some nodes
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    // add a direct edge from v0 to v2
    graph.add_edge([v0, v1, v2])?;
    // find the shortest path from v0 to v2
    let path = graph.dijkstra().find_path(v0, v2)?;
    // verify the results
    assert_eq!(path, [v0, v2], "Direct edge should return direct path");
    // return
    Ok(())
}

#[test]
fn test_dijkstra_shortest_path() -> rshyper::Result<()> {
    // Initialize a new graph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // add vertices to the graph
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    let v4 = graph.add_vertex()?;
    let v5 = graph.add_vertex()?;
    // Add edges to the graph
    graph.add_edge([v0, v1])?;
    graph.add_edge([v1, v2])?;
    graph.add_edge([v2, v5])?;
    graph.add_edge([v5, v4])?;
    graph.add_edge([v0, v4])?;
    graph.add_edge([v4, v3])?;
    graph.add_edge([v0, v5, v3])?;

    // Use Dijkstra's algorithm to find the shortest path from v0 to v3
    let path = graph.dijkstra().find_path(v0, v3)?;
    // Dijkstra should find the shortest path (v0 -> v4 -> v3)
    assert_eq!(
        path,
        [v0, v3],
        "Path should start with v0, go through v4 and end with v3"
    );

    Ok(())
}

#[test]
fn test_dijkstra_no_path() -> rshyper::Result<()> {
    // initializea new undirected hashgraph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // add vertices to the graph
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    // Add edges to the graph
    let e0 = graph.add_edge([v0, v1])?;
    // v2 and v3 are not connected to v0 or v1
    let e1 = graph.add_edge([v0, v2])?;
    // v2 and v3 are disconnected from v0 and v1
    // find the shortest path between v0 and v3
    let path = graph.dijkstra().find_path(v0, v3);
    // verify that there is no path between v0 and v3
    assert!(
        path.is_err() || path.as_ref().unwrap().is_empty(),
        "Should return error or empty path when no path exists"
    );

    Ok(())
}

#[test]
fn test_dijkstra_same_start_end() -> rshyper::Result<()> {
    // initializea new undirected hashgraph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // setup the graph with some nodes and edges
    rshyper::hypergraph! {
        graph {
            nodes: {
                let v0;
                let v1;
            };
            edges: {
                let _e0: [v0, v1] = 1;
            };
        }
    }
    // compute a self-loop from v0 -> v0
    let path = graph.dijkstra().find_path(v0, v0)?;
    // verify the path has one item, which is the node itself
    assert_eq!(
        path,
        [v0],
        "Path from a node to itself should be just that node"
    );

    Ok(())
}

#[test]
fn test_dijkstra_multiple_paths() -> rshyper::Result<()> {
    // initializea new undirected hashgraph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // add some nodes to the graph
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    let v4 = graph.add_vertex()?;
    // Add edges to the graph
    let e0 = graph.add_edge([v0, v1, v3])?;
    let e1 = graph.add_edge([v0, v2])?;
    let e2 = graph.add_edge([v2, v4])?;
    let e3 = graph.add_edge([v3, v4])?;
    // find the shortest path from v0 to v4
    let path = graph.dijkstra().find_path(v0, v4)?;

    // Both [v0, v2, v4] and [v0, v3, v4] are valid shortest paths
    assert!(
        path == [v0, v2, v4] || path == [v0, v3, v4],
        "Path should be one of the two shortest paths"
    );

    Ok(())
}

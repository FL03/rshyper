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
            };
            edges: {
                let _e0: [v0, v1]; // v0 -> v1
                let _e1: [v1, v2]; // v1 -> v2
                let _e2: [v2, v3]; // v2 -> v3
                let _e3: [v0, v4]; // v0 -> v4
                let _e4: [v4, v3]; // v4 -> v3
            };
        }
    }

    // Use Dijkstra's algorithm to find the shortest path from v0 to v3
    let mut dijkstra = graph.dijkstra();
    let path = dijkstra.find_path(v0, v3)?;

    // Dijkstra should find the shortest path (v0 -> v4 -> v3)
    assert_eq!(
        path,
        [v0, v4, v3],
        "Path should start with v0, go through v1 and v2, and end with v3"
    );

    Ok(())
}

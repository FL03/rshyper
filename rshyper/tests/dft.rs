/*
    appellation: dft <test>
    authors: @FL03
*/
use rshyper::hyper_map::UnHyperMap as HyperGraph;

#[test]
fn test_depth_first_traversal() -> rshyper::HyperResult<()> {
    let mut graph = HyperGraph::<usize, usize>::undirected();

    // Create a simple hypergraph
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;

    let _e1 = graph.add_edge([v0, v1])?;
    let _e2 = graph.add_edge([v1, v2])?;
    let _e3 = graph.add_edge([v2, v3])?;

    let mut dft = graph.dft();
    let path = dft.search(v0)?;

    // Verify traversal follows depth-first pattern
    assert_eq!(path.len(), 4, "Should visit all 4 vertices");
    assert_eq!(path[0], v0, "Should start with v0");

    // All vertices should be visited
    assert!(
        dft.has_visited(&v0)
            && dft.has_visited(&v1)
            && dft.has_visited(&v2)
            && dft.has_visited(&v3)
    );

    Ok(())
}

#[test]
fn test_dft_branching_graph() -> rshyper::HyperResult<()> {
    let mut graph = HyperGraph::<usize, usize>::undirected();

    // Create a branching hypergraph
    //      1
    //     /
    // 0 -+-- 2 -- 4
    //     \
    //      3

    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    let v4 = graph.add_vertex()?;

    let _e1 = graph.add_edge([v0, v1])?;
    let _e2 = graph.add_edge([v0, v2])?;
    let _e3 = graph.add_edge([v0, v3])?;
    let _e4 = graph.add_edge([v2, v4])?;

    let mut dft = graph.dft();
    let path = dft.search(v0)?;

    // Should visit all vertices
    assert_eq!(path.len(), 5, "Should visit all vertices");
    assert_eq!(path[0], v0, "Should start with v0");

    // Check for depth-first behavior by ensuring v4 comes after v2
    // Find the positions of v2 and v4 in the path
    let pos_v2 = path.iter().position(|&v| v == v2);
    let pos_v4 = path.iter().position(|&v| v == v4);
    assert!(pos_v2 < pos_v4, "In DFT, v4 should be visited after v2");

    Ok(())
}

#[test]
fn test_dft_cyclic_graph() -> rshyper::HyperResult<()> {
    let mut graph = HyperGraph::<usize, usize>::undirected();

    // Create a cyclic graph
    // 0 -- 1 -- 2
    // |         |
    // +----3----+

    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;

    let _e1 = graph.add_edge([v0, v1])?;
    let _e2 = graph.add_edge([v1, v2])?;
    let _e3 = graph.add_edge([v2, v3])?;
    let _e4 = graph.add_edge([v3, v0])?;

    let mut dft = graph.dft();
    let path = dft.search(v0)?;

    // Each vertex should be visited exactly once
    assert_eq!(path.len(), 4, "Each vertex should be visited exactly once");

    // All vertices should be visited
    let visited = dft.visited();
    assert!(visited.contains(&v0));
    assert!(visited.contains(&v1));
    assert!(visited.contains(&v2));
    assert!(visited.contains(&v3));

    Ok(())
}

#[test]
fn test_dft_isolated_vertex() -> rshyper::HyperResult<()> {
    let mut graph = HyperGraph::<usize, usize>::undirected();

    // Create a graph with an isolated vertex
    // 0 -- 1    2

    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?; // isolated

    let _e1 = graph.add_edge([v0, v1])?;
    // create a new dft instance
    let mut dft = graph.dft();
    // test search starting from `v0`
    let path = dft.search(v0)?;
    // algotihm should only visit connected vertices
    assert_eq!(path.len(), 2, "Should visit only connected vertices");
    // v2 should not be visited
    assert!(!dft.has_visited(&v2));
    // Starting from isolated vertex should only visit that vertex
    let path_from_isolated = dft.search(v2)?;
    assert_eq!(
        path_from_isolated.len(),
        1,
        "Should contain only the isolated vertex"
    );
    assert_eq!(path_from_isolated[0], v2);
    Ok(())
}

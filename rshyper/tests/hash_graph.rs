/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use rshyper::graphs::hash_graph::HashGraph;
use std::collections::HashSet;

#[test]
fn test_hypergraph() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize>::new();

    // Add some vertices
    let v0 = graph.insert_node_default();
    let v1 = graph.insert_node_default();
    let v2 = graph.insert_node_default();
    let v3 = graph.insert_node_default();

    // Add some hyperedges
    let e1 = graph.insert_edge([v0, v1, v2])?;
    let e2 = graph.insert_edge(vec![v1, v2, v3])?;
    assert_ne!(e1, e2);

    // Get neighbors of vertex v1
    let neighbors = graph.neighbors(v1)?;
    let exp = HashSet::from_iter([v0, v2, v3]);
    assert_eq!(neighbors, exp);

    // verify the degree of vertex v1
    assert_eq!(graph.get_vertex_degree(v1)?, 2);
    // remove vertex v1
    let _ = graph.remove_vertex(v2)?;
    // verify the hypergraph does not contain vertex v2
    assert!(!graph.contains_node(&v2));
    // return
    Ok(())
}

#[test]
fn merge_hyperedge() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize>::new();
    let v0 = graph.insert_node(10);
    let v1 = graph.insert_node(20);
    let v2 = graph.insert_node(30);

    let e1 = graph.insert_edge(vec![v0, v1])?;
    let e2 = graph.insert_edge(vec![v1, v2])?;

    let merged = graph.merge_edges(e1, e2)?;
    let hyperedge = graph.remove_edge(merged)?;
    assert!(hyperedge.contains(&v0));
    assert!(hyperedge.contains(&v1));
    assert!(hyperedge.contains(&v2));
    Ok(())
}

#[test]
fn update_vertex() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize>::new();
    let v0 = graph.insert_node(42);

    // Check initial weight
    let initial_weight = graph.get_vertex_weight(v0)?;
    assert_eq!(initial_weight.weight(), &42);
    // Update the weight
    let _ = graph.set_vertex_weight(v0, 100)?;
    // Check updated weight
    let updated_weight = graph.get_vertex_weight(v0)?;
    assert_eq!(updated_weight.weight(), &100);

    Ok(())
}

#[test]
fn remove_hyperedge() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize>::new();
    let v0 = graph.insert_node(10);
    let v1 = graph.insert_node(20);
    let v2 = graph.insert_node(30);

    let e1 = graph.insert_edge(vec![v0, v1])?;
    let e2 = graph.insert_edge(vec![v1, v2])?;

    // Remove hyperedge e1
    let removed_edge = graph.remove_edge(e1)?;
    assert!(removed_edge.contains(&v0));
    assert!(removed_edge.contains(&v1));

    // Check that the removed edge is no longer in the graph
    assert!(!graph.contains_edge(&e1));
    assert!(graph.contains_edge(&e2));

    Ok(())
}

/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use rshyper::{HashGraph, Index};

#[test]
fn test_hypergraph() -> rshyper::Result<()> {
    use std::collections::HashSet;

    let mut graph = HashGraph::<usize>::new();

    // Add some vertices
    let v0 = graph.add_vertex_default();
    let v1 = graph.add_vertex_default();
    let v2 = graph.add_vertex_default();
    let v3 = graph.add_vertex_default();

    // Add some hyperedges
    let e1 = graph.add_hyperedge([v0, v1, v2]);
    let e2 = graph.add_hyperedge(vec![v1, v2, v3]);
    assert!(e1.is_ok());
    assert!(e2.is_ok());

    // Get neighbors of vertex v1
    let neighbors = graph.get_neighbors(v1)?;
    let exp = [2, 3, 0usize].iter().copied().map(|i| Index::from_value(i));
    assert_eq!(neighbors, HashSet::from_iter(exp));

    // Get degree of vertex v1
    let deg = graph.vertex_degree(v1)?;
    assert_eq!(deg, 2);

    // Remove a vertex
    assert!(graph.remove_vertex(v2).is_ok());
    Ok(())
}

#[test]
fn merge_hyperedge() {
    let mut graph = HashGraph::<usize>::new();
    let v0 = graph.add_vertex(10);
    let v1 = graph.add_vertex(20);
    let v2 = graph.add_vertex(30);

    let e1 = graph.add_hyperedge(vec![v0, v1]).expect("edge e1");
    let e2 = graph.add_hyperedge(vec![v1, v2]).expect("edge e2");

    let merged = graph.merge_hyperedges(e1, e2).expect("merge");
    let hyperedge = graph
        .remove_hyperedge(merged)
        .expect("merged hyperedge missing");
    assert!(hyperedge.contains(&v0));
    assert!(hyperedge.contains(&v1));
    assert!(hyperedge.contains(&v2));
}

#[test]
fn update_vertex() {
    let mut graph = HashGraph::<usize>::new();
    let v0 = graph.add_vertex(42);

    // Check initial weight
    let initial_weight = graph.get_vertex_weight(v0).expect("vertex missing");
    assert_eq!(*initial_weight.weight(), 42);
    // Update the weight
    let res = graph.update_vertex_weight(v0, 100);
    assert!(res.is_ok());

    // Check updated weight
    let updated_weight = graph.get_vertex_weight(v0).expect("weight missing");
    assert_eq!(*updated_weight.weight(), 100);
}

#[test]
fn remove_hyperedge() {
    let mut graph = HashGraph::<usize>::new();
    let v0 = graph.add_vertex(10);
    let v1 = graph.add_vertex(20);
    let v2 = graph.add_vertex(30);

    let e1 = graph.add_hyperedge(vec![v0, v1]).expect("edge e1");
    let e2 = graph.add_hyperedge(vec![v1, v2]).expect("edge e2");

    // Remove hyperedge e1
    let removed_edge = graph.remove_hyperedge(e1).expect("remove edge");
    assert!(removed_edge.contains(&v0));
    assert!(removed_edge.contains(&v1));

    // Check that the removed edge is no longer in the graph
    assert!(!graph.check_hyperedge(&e1));
    assert!(graph.check_hyperedge(&e2));
}

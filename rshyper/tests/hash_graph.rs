/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use rshyper::Weight;
use rshyper::hash_graph::HashGraph;
use std::collections::HashSet;

#[test]
fn test_hash_graph_error() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize, usize>::undirected();

    let e1 = graph.add_edge([]);
    assert!(e1.is_err(), "Adding an empty edge should return an error");
    // return
    Ok(())
}

#[test]
fn test_hash_graph() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize, usize>::undirected();

    // use the macro to add some nodes & edges
    let v0 = graph.add_node(10);
    let v1 = graph.add_node(20);
    let v2 = graph.add_node(30);
    let v3 = graph.add_node(40);
    // add some edges
    let _e1 = graph.add_edge(vec![v0, v1])?;
    let e2 = graph.add_edge(vec![v0, v1, v2])?;
    let e3 = graph.add_edge(vec![v1, v2, v3])?;

    // the order of both edges should be equivalent
    assert_eq!(
        graph.find_order_of_edge(&e2)?,
        graph.find_order_of_edge(&e3)?
    );
    assert_ne!(e2, e3);

    // Get neighbors of vertex v1
    let neighbors = graph.neighbors(&v1)?;
    let exp = HashSet::from_iter([v0, v2, v3]);
    assert_eq!(neighbors, exp);

    // verify the degree of vertex v1
    assert_eq!(graph.get_degree_of_node(&v1), 3);
    // remove vertex v1
    let _ = graph.remove_vertex(&v1)?;
    // verify the hypergraph does not contain vertex v2
    assert!(!graph.contains_node(&v1));
    assert_eq!(graph.get_degree_of_node(&v1), 0);
    // return
    Ok(())
}

#[test]
fn test_merge_hash_edge() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize, usize>::undirected();
    let v0 = graph.add_node(10);
    let v1 = graph.add_node(20);
    let v2 = graph.add_node(30);

    let e1 = graph.add_surface(vec![v0, v1], Weight(10))?;
    let e2 = graph.add_surface(vec![v1, v2], Weight(20))?;

    let merged = graph.merge_edges(&e1, &e2)?;
    let hyperedge = graph.remove_surface(&merged)?;
    assert!(
        hyperedge.contains_vertex(&v0)
            && hyperedge.contains_vertex(&v1)
            && hyperedge.contains_vertex(&v2)
    );
    assert_eq!(hyperedge.weight(), &30);
    Ok(())
}

#[test]
fn test_update_hash_node() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize, usize>::undirected();
    let v0 = graph.add_node(42);

    // Check initial weight
    let initial_weight = graph.get_node(&v0)?;
    assert_eq!(initial_weight.weight(), &Weight(42));
    // Update the weight
    let _ = graph.set_vertex_weight(&v0, 100)?;
    // Check updated weight
    let updated_weight = graph.get_node(&v0)?;
    assert_eq!(**updated_weight.weight(), 100);

    Ok(())
}

#[test]
fn test_remove_hash_edges() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize, usize>::undirected();
    let v0 = graph.add_node(10);
    let v1 = graph.add_node(20);
    let v2 = graph.add_node(30);

    let e1 = graph.add_edge(vec![v0, v1])?;
    let e2 = graph.add_edge(vec![v1, v2])?;

    // Remove hyperedge e1
    let removed_edge = graph.remove_surface(&e1)?;
    assert!(removed_edge.contains_vertex(&v0) && removed_edge.contains_vertex(&v1));

    // Check that the removed edge is no longer in the graph
    assert!(!graph.contains_surface(&e1));
    assert!(graph.contains_surface(&e2));

    Ok(())
}

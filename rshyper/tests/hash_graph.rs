/*
    Appellation: graph <module>
    Contrib: @FL03
*/
#![allow(unused_variables)]

use rshyper::{HashGraph, IntoWeight, Weight};
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
    // Add nodes with weights
    let v0 = graph.add_node(Weight(10))?;
    let v1 = graph.add_node(Weight(20))?;
    let v2 = graph.add_node(Weight(30))?;
    let v3 = graph.add_node(Weight(40))?;
    // Add edges (hyperedges) with vertices
    let e0 = graph.add_edge([v0, v1])?;
    let e1 = graph.add_edge([v0, v1, v2])?;
    let e2 = graph.add_edge([v0, v2, v3])?;
    // the order, or number of vertices, in the hypergraph should be 4
    assert_eq!(graph.order(), 4);
    // the size, or number of edges, in the hypergraph should be 3
    assert_eq!(graph.size(), 3);
    // the size of the first edge should be 2
    assert_eq!(graph.get_edge_order(&e0)?, 2);
    // the size of each edge should be equivalent
    assert_eq!(graph.get_edge_order(&e1)?, graph.get_edge_order(&e2)?);
    // verify the hypergraph contains the vertices
    assert!(graph.contains_node(&v0) && graph.contains_node(&v1));

    // Get neighbors of vertex v1
    let neighbors = graph.find_node_neighbors(&v1)?;
    let exp = HashSet::from_iter([v0, v2]);
    assert_eq!(neighbors, exp);
    // verify the degree of vertex v1
    assert_eq!(graph.get_node_degree(&v1), exp.len());
    // remove vertex v1
    let _ = graph.remove_node(&v1)?;
    // verify the hypergraph does not contain vertex v2
    assert!(!graph.contains_node(&v1));
    assert_eq!(graph.get_node_degree(&v1), 0);
    // return
    Ok(())
}

#[test]
fn test_merge_hash_edge() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize, usize>::undirected();
    let v0 = graph.add_node(Weight(10))?;
    let v1 = graph.add_node(Weight(20))?;
    let v2 = graph.add_node(Weight(30))?;

    let e1 = graph.add_surface(vec![v0, v1], Weight(10))?;
    let e2 = graph.add_surface(vec![v1, v2], Weight(20))?;
    // merge the two edges
    let em = graph.merge_edges(&e1, &e2)?;
    // verify that the two edges used to merge are no longer in the graph
    assert!(!graph.contains_edge(&e1) && !graph.contains_edge(&e2));
    // get the merged edge
    let hyperedge = graph.get_surface(&em)?;
    // check the edge contains all vertices
    assert!(hyperedge.contains(&v0) && hyperedge.contains(&v1) && hyperedge.contains(&v2));
    // check that the merged edge has a weight equal to the sum of the weights of the two edges
    assert_eq!(hyperedge.weight(), &30);
    Ok(())
}

#[test]
fn test_update_hash_node() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize, usize>::undirected();
    let v0 = graph.add_node(Weight(42))?;

    // Check initial weight
    let initial_weight = graph.get_node(&v0)?;
    assert_eq!(initial_weight.weight(), &Weight(42));
    // Update the weight
    let _ = graph.set_node_weight(&v0, Weight(100))?;
    // Check updated weight
    let updated_weight = graph.get_node(&v0)?;
    assert_eq!(**updated_weight.weight(), 100);

    Ok(())
}

#[test]
fn test_remove_hash_edges() -> rshyper::Result<()> {
    let mut graph = HashGraph::<usize, usize>::undirected();
    let v0 = graph.add_node(Weight(10))?;
    let v1 = graph.add_node(Weight(20))?;
    let v2 = graph.add_node(Weight(30))?;

    let e1 = graph.add_edge(vec![v0, v1])?;
    let e2 = graph.add_edge(vec![v1, v2])?;

    // Remove hyperedge e1
    let removed_edge = graph.remove_surface(&e1)?;
    assert!(removed_edge.contains(&v0) && removed_edge.contains(&v1));

    // Check that the removed edge is no longer in the graph
    assert!(!graph.contains_edge(&e1));
    assert!(graph.contains_edge(&e2));

    Ok(())
}

#[test]
fn hash_graph_iter() -> rshyper::Result<()> {
    // initialize a new undirected hash graph
    let mut graph = HashGraph::<usize, usize>::undirected();
    // add some nodes
    let v0 = graph.add_node(10.into_weight())?;
    let v1 = graph.add_node(20.into_weight())?;
    let v2 = graph.add_node(30.into_weight())?;
    let v3 = graph.add_node(40.into_weight())?;
    let v4 = graph.add_node(50.into_weight())?;
    let v5 = graph.add_node(60.into_weight())?;
    let v6 = graph.add_node(70.into_weight())?;
    // add some edges
    let e0 = graph.add_edge([v0, v1, v6])?;
    let e1 = graph.add_edge([v1, v2])?;
    let e2 = graph.add_edge([v2, v3])?;
    let e3 = graph.add_edge([v3, v4])?;
    let e4 = graph.add_edge([v4, v5])?;

    let node_iter = graph.node_iter();
    // verify the node iterator yields the correct number of nodes
    assert_eq!(node_iter.count(), graph.order());

    Ok(())
}

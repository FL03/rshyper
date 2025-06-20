/*
    Appellation: graph <module>
    Contrib: @FL03
*/
#![allow(unused_variables)]

use hashbrown::HashSet;
use rshyper_core::{IntoWeight, Result, Weight};
use rshyper_hmap::HyperMap;

#[test]
fn test_error() -> Result<()> {
    // initialize a new, undirected hash-graph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // try adding an empty edge
    let e1 = graph.add_link([]);
    // verify the error
    assert!(e1.is_err(), "Adding an empty edge should return an error");
    // return
    Ok(())
}

#[test]
fn test_hyper_map() -> Result<()> {
    let mut graph = HyperMap::<usize, usize>::undirected();
    // Add nodes with weights
    let v0 = graph.add_node(10.into_weight())?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    // Add edges (hyperedges) with vertices
    let e0 = graph.add_link([v0, v1])?;
    let e1 = graph.add_link([v0, v1, v2])?;
    let e2 = graph.add_link([v0, v2, v3])?;
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
    assert_eq!(graph.get_node_degree(&v1), 2);
    // remove vertex v1
    let _ = graph.remove_node(&v1)?;
    // verify the hypergraph does not contain vertex v2
    assert!(!graph.contains_node(&v1));
    assert_eq!(graph.get_node_degree(&v1), 0);
    // return
    Ok(())
}

#[test]
fn test_merge_edges() -> Result<()> {
    // initialize a new, undirected hash-graph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // add some nodes with weights
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    // add some surfaces (hyperedges) with vertices
    let e1 = graph.add_edge([v0, v1], 10.into_weight())?;
    let e2 = graph.add_edge([v1, v2], 20.into_weight())?;
    // merge the two edges
    let em = graph.merge_edges(&e1, &e2)?;
    // verify that the two edges used to merge are no longer in the graph
    assert!(!graph.contains_edge(&e1) && !graph.contains_edge(&e2));
    // get the merged edge
    let hyperedge = graph.get_edge(&em)?;
    // check the edge contains all vertices
    assert!(hyperedge.contains(&v0) && hyperedge.contains(&v1) && hyperedge.contains(&v2));
    // check that the merged edge has a weight equal to the sum of the weights of the two edges
    assert_eq!(hyperedge.weight(), &30);
    // finish
    Ok(())
}

#[test]
fn test_node_mut() -> Result<()> {
    // initialize a new, undirected hash-graph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // add a node using the default weight
    let v0 = graph.add_vertex()?;
    // Check initial weight
    assert_eq!(graph.get_node_weight(&v0)?, &0);
    // Update the weight
    let _ = graph.set_node_weight(&v0, Weight(100))?;
    // Check updated weight
    assert_eq!(graph.get_node_weight(&v0)?, &100);
    // finish
    Ok(())
}

#[test]
fn test_remove_edge() -> Result<()> {
    // initialize a new, undirected hash-graph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // add some vertices
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    // create some edges with the vertices
    let e1 = graph.add_link([v0, v1])?;
    let e2 = graph.add_link([v1, v2])?;
    // remove hyperedge e1
    let removed_edge = graph.remove_edge(&e1)?;
    // vertify the contents of the removed edge
    assert!(removed_edge.contains(&v0) && removed_edge.contains(&v1));
    // verify the edge is no longer present in the graph while the other remains
    assert!(!graph.contains_edge(&e1) && graph.contains_edge(&e2));
    // finish
    Ok(())
}

#[test]
fn test_seq_iter() -> Result<()> {
    // initialize a new undirected hash graph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // add some nodes
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    let v4 = graph.add_vertex()?;
    let v5 = graph.add_vertex()?;
    let v6 = graph.add_vertex()?;
    // add some edges
    let e0 = graph.add_link([v0, v1, v6])?;
    let e1 = graph.add_link([v1, v2])?;
    let e2 = graph.add_link([v2, v3])?;
    let e3 = graph.add_link([v3, v4])?;
    let e4 = graph.add_link([v4, v5])?;
    // get a sequential iterator over the nodes
    let iter = graph.seq_iter_points();
    // create an array of ids in the order they should be produced
    let exp = [v0, v1, v2, v3, v4, v5, v6];
    // ensure each element is produced
    for (i, val) in iter.enumerate() {
        // verify the nodes are in the correct order
        assert_eq!(val.id(), &exp[i]);
    }
    // get a sequential iterator over the edges
    let iter = graph.seq_iter_facets();
    // create an array of ids in the order they should be produced
    let exp = [e0, e1, e2, e3, e4];
    // ensure each element is produced
    for (i, val) in iter.enumerate() {
        // verify the nodes are in the correct order
        assert_eq!(val.id(), &exp[i]);
    }
    // finish
    Ok(())
}

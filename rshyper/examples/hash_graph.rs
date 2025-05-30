/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use rshyper::HashGraph;

fn main() -> rshyper::Result<()> {
    let mut graph = HashGraph::<()>::new();

    // Add some vertices
    let v0 = graph.insert_node_default();
    let v1 = graph.insert_node_default();
    let v2 = graph.insert_node_default();
    let v3 = graph.insert_node_default();

    // Add some hyperedges
    let e1 = graph.insert_edge(vec![v0, v1, v2])?;
    println!("Added hyperedge {e1}: {:?}", [v0, v1, v2]);

    let e2 = graph.insert_edge(vec![v1, v2, v3])?;
    println!("Added hyperedge {e2}: {:?}", [v1, v2, v3]);

    // Get neighbors of vertex v1
    let neighbors = graph.neighbors(v1)?;
    println!("Neighbors of {}: {:?}", v1, neighbors);

    // Get degree of vertex v1
    let degree = graph.get_vertex_degree(v1)?;
    println!("Degree of {v1}: {degree}");

    // Remove a vertex
    graph.remove_vertex(v2)?;
    println!("Removed vertex {v2}");

    println!("---------\nFinal graph state: {:?}", graph);
    Ok(())
}

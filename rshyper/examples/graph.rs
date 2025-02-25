/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use rshyper::HyperGraph;

fn main() {
    let mut graph = HyperGraph::new();

    // Add some vertices
    let v0 = graph.add_vertex();
    let v1 = graph.add_vertex();
    let v2 = graph.add_vertex();
    let v3 = graph.add_vertex();

    // Add some hyperedges
    match graph.add_hyperedge(vec![v0, v1, v2]) {
        Ok(edge_id) => println!("Added hyperedge {}: {:?}", edge_id, [v0, v1, v2]),
        Err(e) => println!("Error: {}", e),
    }

    match graph.add_hyperedge(vec![v1, v2, v3]) {
        Ok(edge_id) => println!("Added hyperedge {}: {:?}", edge_id, [v1, v2, v3]),
        Err(e) => println!("Error: {}", e),
    }

    // Get neighbors of vertex v1
    match graph.get_neighbors(v1) {
        Ok(neighbors) => println!("Neighbors of {}: {:?}", v1, neighbors),
        Err(e) => println!("Error: {}", e),
    }

    // Get degree of vertex v1
    match graph.vertex_degree(v1) {
        Ok(degree) => println!("Degree of {}: {}", v1, degree),
        Err(e) => println!("Error: {}", e),
    }

    // Remove a vertex
    match graph.remove_vertex(v2) {
        Ok(()) => println!("Removed vertex {}", v2),
        Err(e) => println!("Error: {}", e),
    }

    println!("Final graph state: {:?}", graph);
}
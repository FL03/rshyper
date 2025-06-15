/*
    Appellation: basic_usage <module>
    Contrib: @FL03
*/
use rshyper::{UnHyperMap, hypergraph};

fn main() -> rshyper::HyperResult<()> {
    // use the macro to insert nodes into the graph
    hypergraph! {
        let mut graph: UnHyperMap::<usize, usize> {
            nodes: {
                let v0;
                let v1 = 2;
                let v2 = 3;
                let v3 = 4;
            };
            edges: {
                let e0: [v0, v1, v2] = 10;
                let e1: [v1, v2, v3];
                let e2: [v0, v1] = 20;
                let e3: [v1, v2] = 25;
            };
        }
    }
    println!("*********\n\tInitial graph state:\n*********\n{graph:?}");
    // merge the two edges e2 and e3 into e4
    let e4 = graph.merge_edges(&e2, &e3)?;
    println!(
        "Merged edges {e2} and {e3} into {e4} with a weight of {}",
        graph.get_edge_weight(&e4)?
    );
    // checkout the neighbors of vertex v1
    let neighbors = graph.find_node_neighbors(&v1)?;
    println!(
        "Found {n} neighbors of {v1}: {neighbors:?}",
        n = neighbors.len()
    );
    // checkout some edge properties
    println!("Edge {e1} has order {n}", n = graph.get_edge_order(&e1)?);
    println!("Edge {e0} has weight {w}", w = graph.get_edge_weight(&e0)?);
    // print the final state of the graph
    println!("*********\n\tFinal graph state:\n*********\n{graph}");
    // finish the example and return ()
    Ok(())
}

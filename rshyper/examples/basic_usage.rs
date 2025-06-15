/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use rshyper::HyperMap;

fn main() -> rshyper::Result<()> {
    // initialize a new instance of a hypergraph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // use the macro to insert nodes into the graph
    rshyper::hypergraph! {
        graph {
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
    println!("*********\nInitial graph state:\n*********\n{:?}", graph);
    let e23 = graph.merge_edges(&e2, &e3)?;
    println!(
        "Merged edges {e2} and {e3} into {e23} with a weight of {}",
        graph.get_edge_weight(&e23)?
    );
    let order_e1 = graph.get_edge_order(&e1)?;
    println!("Edge {e1} has order {order_e1}");
    let e0_weight = graph.get_edge_weight(&e0)?;
    println!("Edge {e0} has weight {e0_weight}");

    println!("*********\nFinal graph state:\n*********\n{:?}", graph);
    Ok(())
}

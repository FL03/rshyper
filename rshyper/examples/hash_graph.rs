/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use rshyper::HashGraph;

fn main() -> rshyper::Result<()> {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(tracing::Level::TRACE)
        .init();
    // initialize a new instance of a hypergraph
    let mut graph = HashGraph::<usize, usize>::undirected();
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
    tracing::info!("Initial graph state: {:?}", graph);

    let order_e1 = graph.get_edge_order(&e1)?;
    tracing::info!("Edge {e1} has order {order_e1}");

    let e0_weight = graph.get_edge_weight(&e0)?;
    tracing::info!("Edge {e0} has weight {e0_weight}");

    let e23 = graph.merge_edges(&e2, &e3)?;
    tracing::info!(
        "Merged edges {e2} and {e3} into {e23} with a weight of {weight}",
        weight = graph.get_edge_weight(&e23)?
    );

    // Get neighbors of vertex v1
    let neighbors = graph.find_node_neighbors(&v1)?;
    tracing::info!(
        "found {n} neighbors of {v1}: {neighbors:?}",
        n = neighbors.len()
    );

    // Get degree of vertex v1
    let degree = graph.get_node_degree(&v1);
    tracing::info!("vertex {v} has a degree of {d}", v = v1, d = degree);

    // Remove a vertex
    graph.remove_node(&v2)?;
    tracing::info!("removed vertex {v}...", v = v2);

    tracing::info!("Final graph state: {:?}", graph);
    Ok(())
}

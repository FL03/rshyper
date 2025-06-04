/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use rshyper::HashGraph;
use rshyper::hash_graph::UndirectedHashGraph as HyperGraph;

fn main() -> rshyper::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(tracing::Level::TRACE)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .init();
    // initialize a new instance of a hypergraph
    let mut graph: HyperGraph<usize, usize> = HashGraph::undirected();
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
                let _e2: [v2, v3];
            };
        }
    }
    tracing::info!("Initial graph state: {:?}", graph);

    let order_e1 = graph.get_edge_order(&e1)?;
    tracing::info!("Edge {e1} has order {order_e1}");

    let e0_weight = graph.get_edge_weight(&e0)?;
    tracing::info!("Edge {e0} has weight {e0_weight}");

    // Get neighbors of vertex v1
    let neighbors = graph.neighbors(&v1)?;
    tracing::info!(
        "found {n} neighbors of {v1}: {neighbors:?}",
        n = neighbors.len()
    );

    // Get degree of vertex v1
    let degree = graph.get_node_degree(&v1);
    println!("vertex {v1} has a degree of {degree}");

    // Remove a vertex
    graph.remove_vertex(&v2)?;
    tracing::info!("removed vertex {v2}...");

    tracing::info!("Final graph state: {:?}", graph);
    Ok(())
}

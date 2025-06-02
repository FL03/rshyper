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
    rshyper::hypernode! {
        graph {
            let v0;
            let v1 = 2;
            let v2 = 3;
            let v3 = 4;
        }
    }

    rshyper::hyperedge! {
        graph {
            let e0 = [v0, v1, v2] => 10;
            let e1 = [v1, v2, v3];
            let e2 = [v2, v3];
        }
    }
    tracing::info!("Initial graph state: {:?}", graph);
    // Add some hyperedges
    println!("Added hyperedge {e0}: {:?}", [v0, v1, v2]);
    println!("Added hyperedge {e1}: {:?}", [v1, v2, v3]);
    println!("Added hyperedge {e2}: {:?}", [v2, v3]);

    // Get neighbors of vertex v1
    let neighbors = graph.neighbors(&v1)?;
    println!("Neighbors of {}: {:?}", v1, neighbors);

    // Get degree of vertex v1
    let degree = graph.get_degree_of_node(&v1);
    println!("Degree of {v1}: {degree}");

    // Remove a vertex
    graph.remove_vertex(&v2)?;
    tracing::info!("removed vertex {v2}...");

    tracing::info!("Final graph state: {:?}", graph);
    Ok(())
}

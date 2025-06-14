/*
    Appellation: hash_graph <bench>
    Contrib: @FL03
*/
use self::ext::*;

use core::hint::black_box;
use criterion::{BatchSize, Criterion};

#[cfg(feature = "rand")]
/// benchmark for adding edges
fn bench_hash_graph_add_edge(c: &mut Criterion) {
    c.bench_function("HashGraph::add_edge", |b| {
        b.iter_batched(
            setup,
            |mut graph| {
                // generates a random edge (as parts) using vertices from 0 to 10
                let (verts, weight) = generate_random_edge::<W>(100);
                // add the edge to the graph
                graph
                    .add_surface(black_box(verts), black_box(weight))
                    .expect("failed to add edge");
            },
            BatchSize::SmallInput,
        )
    });
}
/// benchmark for removing edges
fn bench_hash_graph_remove_edge(c: &mut Criterion) {
    c.bench_function("HashGraph::remove_edge", |b| {
        b.iter_batched(
            setup,
            |mut graph| {
                // Use the next value from the iterator as the weight
                for id in 0..5 {
                    graph
                        .remove_node(black_box(&id))
                        .expect("failed to remove node");
                }
            },
            BatchSize::SmallInput,
        )
    });
}

criterion::criterion_group! {
    benches,
    bench_hash_graph_add_edge,
    bench_hash_graph_remove_edge,
}

criterion::criterion_main! {
    benches
}

#[cfg(feature = "rand")]
mod ext {
    use rshyper::HashGraph;
    pub use rshyper::edge::generate_random_edge;

    /// a constant for the sample size of a benchmark group
    #[allow(dead_code)]
    pub const SAMPLES: usize = 50;
    /// a type alias for the type of weight used to benchmark the [`HashGraph`]
    pub type W = u8;

    /// initialize a new [`HashGraph`] with a predefined structure
    pub fn setup() -> HashGraph<W, W> {
        // initialize a new undirected hash graph
        let mut graph = HashGraph::undirected();
        // add 100 nodes to the graph
        let _verts = graph.add_nodes(0..100).expect("failed to add nodes");
        // add 100 edges to the graph
        for _ in 0..100 {
            // each edge contains between 2 and 100 vertices & a random weight
            let (verts, weight) = generate_random_edge::<W>(100);
            // add a self-loop to each vertex
            graph
                .add_surface(verts, weight)
                .expect("failed to add surface");
        }

        graph
    }
}

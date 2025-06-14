/*
    Appellation: hash_graph <bench>
    Contrib: @FL03
*/
use self::ext::*;
use rshyper::VertexId;
use rshyper::hash_graph::HashGraph;

use core::hint::black_box;
use criterion::{BatchSize, BenchmarkId, Criterion, Throughput};

/// benchmark for adding nodes to the graph
fn bench_hash_graph_add_node(c: &mut Criterion) {
    c.bench_function("HashGraph::add_nodes", |b| {
        b.iter_batched(
            setup,
            |mut graph| {
                graph
                    .add_nodes(black_box(0..100))
                    .expect("failed to add nodes")
            },
            BatchSize::SmallInput,
        )
    });
}
/// benchmark calculating the degree of a node
fn bench_hash_graph_get_node_degree(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashGraph::get_node_degree");
    for tgt in 0..10 {
        group.throughput(Throughput::Elements(tgt as u64));
        group.bench_with_input(BenchmarkId::from_parameter(tgt), &tgt, |b, &n| {
            b.iter_batched(
                setup,
                |graph| {
                    // Simulate some operation with the graph
                    graph.get_node_degree(&VertexId::from(n));
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_hash_graph_find_node_neighbors(c: &mut Criterion) {
    // initialize the benchmark group
    let mut group = c.benchmark_group("HashGraph::find_node_neighbors");
    // configure the sample size for the group
    group.sample_size(SAMPLES);
    for tgt in 0..10 {
        group.throughput(Throughput::Elements(tgt as u64));
        group.bench_with_input(BenchmarkId::from_parameter(tgt), &tgt, |b, &n| {
            b.iter_batched(
                setup,
                |graph| {
                    // Simulate some operation with the graph
                    graph
                        .find_node_neighbors(&VertexId::from(n))
                        .expect("failed to find any neighbors")
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_hash_graph_remove_node(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashGraph");
    // set the sample size for the group
    group.sample_size(SAMPLES);
    // benchmark the function
    group.bench_function("remove_node", |b| {
        b.iter_batched(
            || {
                let mut graph = HashGraph::<usize, usize>::undirected();
                let verts = graph.add_nodes(0..100).expect("failed to add nodes");
                (graph, verts)
            },
            |(mut graph, verts)| {
                // remove all the created nodes
                for id in verts.iter() {
                    graph
                        .remove_node(black_box(id))
                        .expect("failed to remove node");
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

criterion::criterion_group! {
    benches,
    bench_hash_graph_add_node,
    bench_hash_graph_get_node_degree,
    bench_hash_graph_find_node_neighbors,
    bench_hash_graph_remove_node,
}

criterion::criterion_main! {
    benches
}

#[cfg(feature = "rand")]
mod ext {
    use rshyper::HashGraph;
    use rshyper::edge::generate_random_edge;

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

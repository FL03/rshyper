/*
    Appellation: hash_graph <bench>
    Contrib: @FL03
*/
use self::ext::*;

use core::hint::black_box;
use criterion::{BatchSize, BenchmarkId, Criterion, Throughput};

/// benchmark for breadth-first traversal search in the [`HashGraph`]
fn benchmark_hg_search_bft(c: &mut Criterion) {
    // initialize the benchmark group
    let mut group = c.benchmark_group("HashGraph::bft");
    // set the sample size for the group
    group.sample_size(SAMPLES);
    // iterator  over a range of target vertices to benchmark
    for tgt in 0..10 {
        group.throughput(Throughput::Elements(tgt as u64));
        group.bench_with_input(BenchmarkId::from_parameter(tgt), &tgt, |b, &n| {
            b.iter_batched(
                setup,
                |graph| {
                    let id = n.into();
                    // search the graph for some target vertex
                    graph.bft().search(black_box(id)).unwrap();
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}
/// benchmark for depth-first traversal search in the [`HashGraph`]
fn hash_graph_bench_search_dft(c: &mut Criterion) {
    // create a benchmark group for depth-first traversal
    let mut group = c.benchmark_group("HashGraph::dft");
    // set the sample size for the group
    group.sample_size(SAMPLES);
    // iterate over a range of target vertices to benchmark
    for tgt in 0..10 {
        group.throughput(Throughput::Elements(tgt as u64));

        group.bench_with_input(BenchmarkId::from_parameter(tgt), &tgt, |b, &n| {
            b.iter_batched(
                setup,
                |graph| {
                    let id = n.into();
                    // search the graph for some target vertex
                    graph.dft().search(black_box(id)).unwrap();
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion::criterion_group! {
    benches,
    benchmark_hg_search_bft,
    hash_graph_bench_search_dft,
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

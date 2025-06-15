/*
    Appellation: hash_graph <bench>
    Contrib: @FL03
*/
use self::ext::*;
use rshyper::VertexId;

use core::hint::black_box;
use criterion::{BatchSize, Criterion};

/// benchmark various edge operations on the [`HashGraph`] implementation.
fn bench_hash_graph_edges(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashGraph::edges");
    // set the sample size for the group
    group.sample_size(SAMPLES);
    // set the duration for the measurement
    group.measurement_time(std::time::Duration::from_secs(DURATION));
    // benchmark the `add_edge` function
    group.bench_function("add_edge", |b| {
        b.iter_batched(
            setup,
            |mut graph| {
                // generates a random edge (as parts) using vertices from 0 to 10
                let (verts, weight) = generate_random_edge::<Wt>(N);
                // add the edge to the graph
                graph
                    .add_surface(black_box(verts), black_box(weight))
                    .expect("failed to add edge");
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the `remove_edge` function
    group.bench_function("remove_edge", |b| {
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
    // finish the group
    group.finish();
}

/// benchmark for the [`HashGraph`] implementation.
fn bench_hash_graph_nodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashGraph::nodes");
    // set the sample size for the group
    group.sample_size(SAMPLES);
    // set the duration for the measurement
    group.measurement_time(std::time::Duration::from_secs(DURATION));
    // benchmark the `add_nodes` function
    group.bench_function("add_nodes", |b| {
        b.iter_batched(
            setup,
            |mut graph| graph.add_nodes(black_box(0..100)).collect::<Vec<_>>(),
            BatchSize::SmallInput,
        )
    });
    // benchmark the `get_node_degree` function
    group.bench_function("get_node_degree", |b| {
        b.iter_batched(
            setup,
            |graph| {
                let i = rand::random_range(0..(N as u128)) % 100;
                let idx = VertexId::from(i as usize);
                // get the degree of each node
                graph.get_node_degree(black_box(&idx));
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the `find_node_neighbors` function
    group.bench_function("find_node_neighbors", |b| {
        b.iter_batched(
            setup,
            |graph| {
                let i = rand::random_range(0..(N as u128)) % 100;
                let idx = VertexId::from(i as usize);
                // get the degree of each node
                graph
                    .find_node_neighbors(black_box(&idx))
                    .expect("failed to find node neighbors");
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the `remove_node` function
    group.bench_function("remove_node", |b| {
        b.iter_batched(
            setup,
            |mut graph| {
                graph.history().nodes().clone().iter().for_each(|id| {
                    graph
                        .remove_node(black_box(id))
                        .expect("failed to remove node");
                })
            },
            BatchSize::SmallInput,
        )
    });
    // finish the group
    group.finish();
}

/// benchmarks for search algorithms in the [`HashGraph`] implementation.
fn bench_hash_graph_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashGraph::search");
    // set the sample size for the group
    group.sample_size(SAMPLES);
    // set the duration for the measurement
    group.measurement_time(std::time::Duration::from_secs(DURATION));
    // benchmark the breadth-first traversal search
    group.bench_function("A*", |b| {
        b.iter_batched(
            setup,
            |graph| {
                let idx = VertexId::random_between(0..N);
                // get the degree of each nodelet id = n.into();
                // search the graph for some target vertex
                graph.astar(hue::<f64>).search(black_box(idx)).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the breadth-first traversal search
    group.bench_function("bft", |b| {
        b.iter_batched(
            setup,
            |graph| {
                let idx = VertexId::random_between(0..N);
                // get the degree of each nodelet id = n.into();
                // search the graph for some target vertex
                graph.bft().search(black_box(idx)).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the depth-first traversal search
    group.bench_function("dft", |b| {
        b.iter_batched(
            setup,
            |graph| {
                let idx = VertexId::random_between(0..N);
                // get the degree of each nodelet id = n.into();
                // search the graph for some target vertex
                graph.dft().search(black_box(idx)).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
    // finish the group
    group.finish();
}

criterion::criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(SAMPLES).measurement_time(std::time::Duration::from_secs(DURATION)).with_plots();
    targets = bench_hash_graph_edges, bench_hash_graph_nodes, bench_hash_graph_search
}

criterion::criterion_main! {
    benches
}

#[cfg(feature = "rand")]
mod ext {
    use rshyper::{HashGraph, VertexId};
    // re-import the generate_random_edge function for convenience
    pub(crate) use rshyper::edge::generate_random_edge;

    /// the duration, in seconds, for which the benchmarks should run
    pub const DURATION: u64 = 7;
    /// a constant for the sample size of a benchmark group
    pub const SAMPLES: usize = 50;
    /// the number of initialized nodes setup by the [`setup`] method
    pub const N: usize = 100;
    /// a type alias for the type of weight used to benchmark the [`HashGraph`]
    pub type Wt = i128;

    /// initialize a new [`HashGraph`] with a predefined structure
    pub fn setup() -> HashGraph<Wt, Wt> {
        // initialize a new undirected hash graph
        let mut graph = HashGraph::<Wt, Wt>::undirected();
        // add 100 nodes to the graph
        let _ = graph.add_nodes(0..(N as Wt)).collect::<Vec<_>>();
        // add 100 edges to the graph
        for _ in 0..graph.order() {
            // each edge contains between 2 and 100 vertices & a random weight
            let (verts, weight) = generate_random_edge::<Wt>(N);
            // add a self-loop to each vertex
            graph
                .add_surface(verts, weight)
                .expect("failed to add surface");
        }

        graph
    }

    // a dummy hueristic function that returns a constant value
    pub fn hue<T>(_a: VertexId, _b: VertexId) -> T
    where
        T: num_traits::One,
    {
        // a dummy heuristic function that returns a constant value
        T::one()
    }
}

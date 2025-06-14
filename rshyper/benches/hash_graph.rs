/*
    Appellation: hash_graph <bench>
    Contrib: @FL03
*/
use self::ext::*;
use rshyper::VertexId;

use core::hint::black_box;
use criterion::{BatchSize, Criterion};

/// benchmark for adding edges
fn bench_hash_graph_edge(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashGraph");
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
    group.bench_function("HashGraph::remove_edge", |b| {
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
    // finish the benchmark group
    group.finish()
}

/// benchmark calculating the degree of a node
fn bench_hash_graph_node(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashGraph");
    // set the sample size for the group
    group.sample_size(SAMPLES);
    // set the duration for the measurement
    group.measurement_time(std::time::Duration::from_secs(DURATION));
    // benchmark the `add_nodes` function
    group.bench_function("add_nodes", |b| {
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
                    graph.find_node_neighbors(black_box(&idx))
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
                if let Some(verts) = graph.history().vertex_history().cloned()  {
                    verts.iter().for_each(|id| {
                        graph
                            .remove_node(black_box(id))
                            .expect("failed to remove node");
                    })
                }


            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

/// benchmark for breadth-first traversal search in the [`HashGraph`]
fn bench_hash_graph_search(c: &mut Criterion) {
    // initialize the benchmark group
    let mut group = c.benchmark_group("HashGraph");
    // set the sample size for the group
    group.sample_size(SAMPLES);
    // set the duration for the measurement
    group.measurement_time(std::time::Duration::from_secs(DURATION));
    // benchmark the breadth-first traversal search
    group.bench_function("bft", |b| {
        b.iter_batched(
            setup,
            |graph| {
                    let i = rand::random_range(0..(N as u128)) % 100;
                    let idx = VertexId::from(i as usize);
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
                    let i = rand::random_range(0..(N as u128)) % 100;
                    let idx = VertexId::from(i as usize);
                    // get the degree of each nodelet id = n.into();
                    // search the graph for some target vertex
                    graph.dft().search(black_box(idx)).unwrap();
                },

            BatchSize::SmallInput,
        )
    });
    group.finish();
}

criterion::criterion_group! {
    benches,
    bench_hash_graph_edge,
    bench_hash_graph_node,
    bench_hash_graph_search,
}

criterion::criterion_main! {
    benches
}

#[cfg(feature = "rand")]
mod ext {
    use rshyper::HashGraph;
    pub(crate) use rshyper::edge::generate_random_edge;

    pub const DURATION: u64 = 7;
    /// a constant for the sample size of a benchmark group
    pub const SAMPLES: usize = 50;

    pub const N: usize = 100;
    /// a type alias for the type of weight used to benchmark the [`HashGraph`]
    pub type Wt = u8;

    /// initialize a new [`HashGraph`] with a predefined structure
    pub fn setup() -> HashGraph<Wt, Wt> {
        // initialize a new undirected hash graph
        let mut graph = HashGraph::undirected();
        // add 100 nodes to the graph
        let _verts = graph.add_nodes(0..(N as Wt)).expect("failed to add nodes");
        // add 100 edges to the graph
        for _ in 0..N {
            // each edge contains between 2 and 100 vertices & a random weight
            let (verts, weight) = generate_random_edge::<Wt>(N);
            // add a self-loop to each vertex
            graph
                .add_surface(verts, weight)
                .expect("failed to add surface");
        }

        graph
    }
}

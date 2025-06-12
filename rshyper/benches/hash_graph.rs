/*
    Appellation: hash_graph <bench>
    Contrib: @FL03
*/
use rshyper::hash_graph::{HashGraph, VertexSet};
use rshyper::{VertexId, Weight};

use core::hint::black_box;
use criterion::{BatchSize, BenchmarkId, Criterion, Throughput};

const SAMPLE_SIZE: usize = 50;

fn _init() -> HashGraph<usize, usize> {
    let mut graph = HashGraph::undirected();

    rshyper::hypergraph! {
        graph {
            nodes: {
                let v0 = 0;
                let v1 = 1;
                let v2 = 2;
                let v3 = 3;
                let v4 = 4;
                let v5 = 5;
                let v6 = 6;
                let v7 = 7;
                let v8 = 8;
                let v9 = 9;
            };
            edges: {
                let _e0: [v0, v1, v2] = 10;
                let _e1: [v0, v3, v4] = 20;
                let _e2: [v0, v5, v6] = 30;
                let _e3: [v1, v2, v3] = 40;
                let _e4: [v1, v4, v5] = 50;
                let _e5: [v1, v6, v7] = 60;
                let _e6: [v2, v3, v4] = 70;
                let _e7: [v2, v5, v6] = 80;
                let _e8: [v2, v7, v8] = 90;
                let _e9: [v3, v4, v5] = 100;
                let _e10: [v3, v6, v7] = 110;
                let _e11: [v3, v8, v9] = 120;
            };
        }
    }

    graph
}

lazy_static::lazy_static! {
    static ref GRAPH: HashGraph<usize, usize> = _init();
}

/// benchmark for adding edges
fn hash_graph_bench_edge_add(c: &mut Criterion) {
    c.bench_function("HashGraph::add_edge", |b| {
        b.iter_batched(
            HashGraph::<usize, usize>::undirected,
            |mut graph| {
                // generate some set of three vertices
                // Use the next value from the iterator as the weight
                for w in 0..100 {
                    let verts = (0..3)
                        .map(|i| VertexId::from((i + i * 2) % 10))
                        .collect::<Vec<_>>();
                    black_box(
                        graph
                            .add_surface(verts, Weight(w))
                            .expect("failed to add edge"),
                    );
                }
            },
            BatchSize::SmallInput,
        )
    });
}
/// benchmark for removing edges
fn hash_graph_bench_edge_remove(c: &mut Criterion) {
    c.bench_function("HashGraph::remove_edge", |b| {
        b.iter_batched(
            _init,
            |mut graph| {
                // Use the next value from the iterator as the weight
                for id in 0..5 {
                    black_box(graph.remove_node(&id).expect("failed to remove node"));
                }
            },
            BatchSize::SmallInput,
        )
    });
}
/// benchmark for adding nodes to the graph
fn hash_graph_bench_node_add(c: &mut Criterion) {
    c.bench_function("HashGraph::add_nodes", |b| {
        b.iter_batched(
            _init,
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
fn hash_graph_bench_node_degree(c: &mut Criterion) {
    let graph = GRAPH.clone();
    let compute = |id: usize| {
        // Simulate some operation with the graph
        graph.get_node_degree(&VertexId::from(id));
    };

    let mut group = c.benchmark_group("HashGraph::get_node_degree");
    for tgt in 0..10 {
        group.throughput(Throughput::Elements(tgt as u64));
        group.bench_with_input(BenchmarkId::from_parameter(tgt), &tgt, |b, &n| {
            b.iter(|| compute(n));
        });
    }
    group.finish();
}

fn hash_graph_bench_node_neighbors(c: &mut Criterion) {
    let graph = GRAPH.clone();
    let compute = |id: usize| -> VertexSet {
        // Simulate some operation with the graph
        graph
            .find_node_neighbors(&VertexId::from(id))
            .expect("failed to find any neighbors")
    };

    let mut group = c.benchmark_group("HashGraph::find_node_neighbors");
    group.sample_size(SAMPLE_SIZE);
    for tgt in 0..10 {
        group.throughput(Throughput::Elements(tgt as u64));
        group.bench_with_input(BenchmarkId::from_parameter(tgt), &tgt, |b, &n| {
            b.iter(|| compute(n));
        });
    }
    group.finish();
}

fn bench_hasg_graph_remove_node(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashGraph");
    // set the sample size for the group
    group.sample_size(SAMPLE_SIZE);
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
}

fn hash_graph_bench_search_bft(c: &mut Criterion) {
    let graph = GRAPH.clone();
    let mut group = c.benchmark_group("HashGraph::bft");
    for tgt in 0..10 {
        group.throughput(Throughput::Elements(tgt as u64));
        group.bench_with_input(BenchmarkId::from_parameter(tgt), &tgt, |b, &n| {
            b.iter(|| {
                let id = n.into();
                // search the graph for some target vertex
                graph.bft().search(black_box(id)).unwrap();
            });
        });
    }
    group.finish();
}

fn hash_graph_bench_search_dft(c: &mut Criterion) {
    // clone the graph to ensure we have a fresh instance for each benchmark
    let graph = GRAPH.clone();
    // create a benchmark group for depth-first traversal
    let mut group = c.benchmark_group("HashGraph::dft");
    // set the sample size for the group
    group.sample_size(SAMPLE_SIZE);
    // iterate over a range of target vertices to benchmark
    for tgt in 0..10 {
        group.throughput(Throughput::Elements(tgt as u64));

        group.bench_with_input(BenchmarkId::from_parameter(tgt), &tgt, |b, &n| {
            b.iter(|| {
                let id = n.into();
                // search the graph for some target vertex
                graph.dft().search(black_box(id)).unwrap();
            });
        });
    }
    group.finish();
}
criterion::criterion_group! {
    benches,
    hash_graph_bench_edge_add,
    hash_graph_bench_edge_remove,
    hash_graph_bench_node_add,
    hash_graph_bench_node_degree,
    hash_graph_bench_node_neighbors,
    bench_hasg_graph_remove_node,
    hash_graph_bench_search_bft,
    hash_graph_bench_search_dft,
}

criterion::criterion_main! {
    benches
}

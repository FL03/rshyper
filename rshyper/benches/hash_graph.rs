/*
    Appellation: hash_graph <bench>
    Contrib: @FL03
*/
use rshyper::hash_graph::HashGraph;
use rshyper::{VertexId, Weight};

use core::hint::black_box;
use criterion::{BatchSize, BenchmarkId, Criterion, Throughput};

const SAMPLE_SIZE: usize = 50;

type WeightType = u8;

fn _init() -> HashGraph<WeightType, WeightType> {
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

#[cfg(feature = "rand")]
/// returns a random edge for the graph by generating anywhere between `2` and `n` vertices and a
/// random weight
fn generate_random_edge<E>(n: usize) -> (Vec<VertexId>, Weight<E>)
where
    E: Clone + Into<usize>,
    rand_distr::StandardUniform: rand_distr::Distribution<E>,
{
    use rand::Rng;
    let mut rng = rand::rng();
    // generate a random set of vertices containing anywhere between 2 and n vertices
    let verts = (0..(rng.random_range(2..=n)))
        .map(|_| VertexId::from(rng.random_range(0..n)))
        .collect::<Vec<_>>();
    let weight = Weight(rng.random());
    (verts, weight)
}

lazy_static::lazy_static! {
    static ref GRAPH: HashGraph<WeightType, WeightType> = _init();
}

#[cfg(feature = "rand")]
/// benchmark for adding edges
fn bench_hash_graph_add_edge(c: &mut Criterion) {
    c.bench_function("HashGraph::add_edge", |b| {
        b.iter_batched(
            _init,
            |mut graph| {
                // generates a random edge (as parts) using vertices from 0 to 10
                let (verts, weight) = generate_random_edge::<WeightType>(10);
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
            _init,
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
/// benchmark for adding nodes to the graph
fn bench_hash_graph_add_node(c: &mut Criterion) {
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
fn bench_hash_graph_get_node_degree(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashGraph::get_node_degree");
    for tgt in 0..10 {
        group.throughput(Throughput::Elements(tgt as u64));
        group.bench_with_input(BenchmarkId::from_parameter(tgt), &tgt, |b, &n| {
            b.iter_batched(
                _init,
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
    group.sample_size(SAMPLE_SIZE);
    for tgt in 0..10 {
        group.throughput(Throughput::Elements(tgt as u64));
        group.bench_with_input(BenchmarkId::from_parameter(tgt), &tgt, |b, &n| {
            b.iter_batched(
                _init,
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
    group.finish();
}

fn hash_graph_bench_search_bft(c: &mut Criterion) {
    // initialize the benchmark group
    let mut group = c.benchmark_group("HashGraph::bft");
    // set the sample size for the group
    group.sample_size(SAMPLE_SIZE);
    // iterator  over a range of target vertices to benchmark
    for tgt in 0..10 {
        group.throughput(Throughput::Elements(tgt as u64));
        group.bench_with_input(BenchmarkId::from_parameter(tgt), &tgt, |b, &n| {
            b.iter_batched(
                _init,
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

fn hash_graph_bench_search_dft(c: &mut Criterion) {
    // create a benchmark group for depth-first traversal
    let mut group = c.benchmark_group("HashGraph::dft");
    // set the sample size for the group
    group.sample_size(SAMPLE_SIZE);
    // iterate over a range of target vertices to benchmark
    for tgt in 0..10 {
        group.throughput(Throughput::Elements(tgt as u64));

        group.bench_with_input(BenchmarkId::from_parameter(tgt), &tgt, |b, &n| {
            b.iter_batched(
                _init,
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
    bench_hash_graph_add_edge,
    bench_hash_graph_remove_edge,
    bench_hash_graph_add_node,
    bench_hash_graph_get_node_degree,
    bench_hash_graph_find_node_neighbors,
    bench_hash_graph_remove_node,
    hash_graph_bench_search_bft,
    hash_graph_bench_search_dft,
}

criterion::criterion_main! {
    benches
}

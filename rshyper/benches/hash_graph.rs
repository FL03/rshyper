/*
    Appellation: hash_graph <bench>
    Contrib: @FL03
*/
use criterion::{Criterion, criterion_group, criterion_main};
use rshyper::{HashGraph, VertexId};
use std::hint::black_box;

fn _init() -> HashGraph<&'static str, usize> {
    let mut graph = HashGraph::new();
    let a = graph.insert_node("A");
    let b = graph.insert_node("B");
    let c = graph.insert_node("C");

    let _e1 = graph
        .insert_edge_with_weight([a, b, c], 0)
        .expect("Failed to insert edge");
    graph
}

fn bench_hash_graph_depth_first(c: &mut Criterion) {
    c.bench_function("hash_graph_dft", |b| {
        b.iter(|| {
            let graph = _init();
            black_box(graph.dft().search(VertexId::from(0)).unwrap());
        })
    });
}

// initialize the benchmark group
criterion_group! {
    benches,
    bench_hash_graph_depth_first,
}
// This macro expands to a function named `benches`, which uses the given config
criterion_main!(benches);

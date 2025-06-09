/*
    Appellation: hash_graph <bench>
    Contrib: @FL03
*/
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use rshyper::hash_graph::UndirectedHashGraph as HyperGraph;
use rshyper::{EdgeId, VertexId};
use std::hint::black_box;

fn _init() -> rshyper::Result<HyperGraph<&'static str, usize>> {
    let mut graph = HyperGraph::undirected();

    rshyper::hypergraph! {
        graph {
            nodes: {
                let a = "A";
                let b = "B";
                let c = "C";
            };
            edges: {
                let _e1: [a, b, c] = 100;
            };
        }
    }

    Ok(graph)
}

fn bench_hash_graph_add_node(c: &mut Criterion) {
    c.bench_function("hash_graph_add_node", |b| {
        b.iter_batched(
            || _init().unwrap(),
            |mut graph| {
                black_box(graph.add_node("D").unwrap());
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_hash_graph_add_edge(c: &mut Criterion) {
    c.bench_function("hash_graph_add_edge", |b| {
        b.iter_batched(
            || {
                let mut graph = _init().unwrap();
                let d = graph.add_node("D").unwrap();
                (graph, d)
            },
            |(mut graph, d)| {
                let a = VertexId::from(0);
                let b = VertexId::from(1);
                black_box(graph.add_edge([a, b, d]).unwrap());
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_hash_graph_remove_node(c: &mut Criterion) {
    c.bench_function("hash_graph_remove_node", |b| {
        b.iter_batched(
            || _init().unwrap(),
            |mut graph| {
                let a = VertexId::from(0);
                black_box(graph.remove_node(&a).unwrap());
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_hash_graph_remove_edge(c: &mut Criterion) {
    c.bench_function("hash_graph_remove_edge", |b| {
        b.iter_batched(
            || _init().unwrap(),
            |mut graph| {
                let e = EdgeId::from(0);
                black_box(graph.remove_surface(&e).unwrap());
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_hash_graph_bft(c: &mut Criterion) {
    c.bench_function("hash_graph_bft", |b| {
        b.iter(|| {
            let graph = _init().expect("Failed to initialize graph");
            black_box(graph.bft().search(VertexId::from(0)).unwrap());
        })
    });
}

fn bench_hash_graph_dft(c: &mut Criterion) {
    c.bench_function("hash_graph_dft", |b| {
        b.iter(|| {
            let graph = _init().expect("Failed to initialize graph");
            black_box(graph.dft().search(VertexId::from(0)).unwrap());
        })
    });
}

fn bench_hash_graph_neighbors(c: &mut Criterion) {
    c.bench_function("hash_graph_neighbors", |b| {
        b.iter(|| {
            let graph = _init().unwrap();
            black_box(graph.find_node_neighbors(&VertexId::from(0)).unwrap());
        })
    });
}

fn bench_hash_graph_degree(c: &mut Criterion) {
    c.bench_function("hash_graph_degree", |b| {
        b.iter(|| {
            let graph = _init().unwrap();
            black_box(graph.get_node_degree(&VertexId::from(0)));
        })
    });
}

criterion_group! {
    benches,
    bench_hash_graph_add_node,
    bench_hash_graph_add_edge,
    bench_hash_graph_remove_node,
    bench_hash_graph_remove_edge,
    bench_hash_graph_bft,
    bench_hash_graph_dft,
    bench_hash_graph_neighbors,
    bench_hash_graph_degree,
}
criterion_main!(benches);

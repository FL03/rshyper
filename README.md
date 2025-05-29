# rshyper

[![crates.io](https://img.shields.io/crates/v/rshyper?style=for-the-badge&logo=rust)](https://crates.io/crates/rshyper)
[![docs.rs](https://img.shields.io/docsrs/rshyper?style=for-the-badge&logo=docs.rs)](https://docs.rs/rshyper)
[![GitHub License](https://img.shields.io/github/license/FL03/rshyper?style=for-the-badge&logo=github)](https://github.com/FL03/rshyper/blob/main/LICENSE)

***

_**Warning: The library is currently in the early stages of development and is not yet ready for production use.**_

`rshyper` is a Rust library designed to provide a hypergraph implementation with a focus on performance and flexibility. It is built to handle complex relationships between data points efficiently, making it suitable for various applications in graph theory, data analysis, and more.

## Background

Before diving in to the technical side of things, let's start by defining several terms commonly used in the definition and implementation of hypergraphs.

- **edge**: an edge is a connection between two or more vertices.
- **facet**: a facet materializes a hyperedge by associating some weight with the edge.
- **node**: a node is a complete _vertex_ in that it is considered to be weighted.
- **vertex**: a vertex can be understood as a _point_ in space that is used to define edges within a hypergraph.

### Hypergraphs

#### Definition

Formally, a hypergraph is defined as a pair $H = (V, E)$ where:

- $V$ is a set of vertices (or nodes).
- $E$ is a set of hyperedges, where each hyperedge is a subset of $V$ that can contain one or more vertices.

## Features

- [x] `hash` - A hash-based hypergraph implementation.

## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com//rshyper.git
cd rshyper
```

#### _Building the project_

```bash
cargo build --all-features -r -v --workspace
```

#### _Running tests_

```bash
cargo test --all-features -r -v --workspace
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.rshyper]
features = []
version = "0.0.x"
```

### Examples

#### _Example #1:_ Basic Usage

```rust
    extern crate rshyper;

    fn main() -> anyhow::Result<()> {
        let mut graph = HashGraph::<()>::new();

        // Add some vertices
        let v0 = graph.add_vertex_default();
        let v1 = graph.add_vertex_default();
        let v2 = graph.add_vertex_default();
        let v3 = graph.add_vertex_default();

        // add hyperedges
        let e1 = graph.add_hyperedge(vec![v0, v1, v2])?;
        let e2 = graph.add_hyperedge(vec![v1, v2, v3])?;
        // Get neighbors of vertex v1
        let neighbors = graph.get_neighbors(v1)?;
        // Get degree of vertex v1
        let degree = graph.vertex_degree(v1)?;
        // Remove a vertex
        graph.remove_vertex(v2)?;
        println!("\n****** Final Graph State ******\n{state:?}", state = graph);
        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

# rshyper

[![crates.io](https://img.shields.io/crates/v/rshyper?style=for-the-badge&logo=rust)](https://crates.io/crates/rshyper)
[![docs.rs](https://img.shields.io/docsrs/rshyper?style=for-the-badge&logo=docs.rs)](https://docs.rs/rshyper)
[![GitHub License](https://img.shields.io/github/license/FL03/rshyper?style=for-the-badge&logo=github)](https://github.com/FL03/rshyper/blob/main/LICENSE)

***

_**Warning: The library is currently in the early stages of development and is not yet ready for production use.**_

`rshyper` is a Rust library designed to provide a hypergraph implementation with a focus on performance and flexibility. It is built to handle complex relationships between data points efficiently, making it suitable for various applications in graph theory, data analysis, and more.

## Table of Contents

- [Background](#background)
- [Usage](#usage)
  - [Features](#features)
  - [Examples](#examples)

## Background

Hypergraphs are generalizations of traditional graphs that allow edges to connect any number of vertices, rather than just two. This flexibility makes hypergraphs suitable for modeling complex relationships in various domains, such as social networks, biological systems, and data analysis.

- [Terminology](#terminology)
- [Hypergraphs](#hypergraphs)
  - [Definition](#definition)
  - [Properties](#properties)

### Terminology

Before diving in to the technical side of things, let's start by defining several terms commonly used in the definition and implementation of hypergraphs.

- **edge**: here, we consider a hyperedge to specifically define
- **facet**: a facet materializes a hyperedge by associating some weight with the edge.
- **node**: a node is a complete _vertex_ in that it is considered to be weighted.
- **surface**: here, the terms surface and facet are used interchangeably, and they refer to the same concept of a hyperedge with an associated weight.
- **vertex**: a vertex can be understood as a _point_ in space that is used to define edges within a hypergraph.

### Hypergraphs

A hypergraph is an abstract data-structure that generalizes the concept of a graph (and even that of the simplicial complex). In a hypergraph, edges (called hyperedges) can connect any number of vertices, allowing for more complex relationships than traditional graphs.

#### Definition

Formally, a hypergraph is defined as a pair $H = (V, E)$ where:

- $V$ is a set of vertices (or nodes).
- $E$ is a set of hyperedges, where each hyperedge is a subset of $V$ that can contain one or more vertices such that $E \subseteq 2^V$.

#### Properties

Listed below are some intrinsic properties of hypergraphs:

- **order:** The order of a hypergraph `H` is the number of vertices in `V`.
- **size:** The size of a hypergraph `H` is the number of hyperedges in `E`.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.rshyper]
features = [
    "hash_graph",
    "macros",
]
version = "0.1.x"
```

### Features

The `rshyper` library provides several features to enhance and isolate its functionality:

- `hash_graph` - A hash-based hypergraph implementation.
- `macros` - A set of macros to simplify hypergraph creation and manipulation.

#### Dependency related features

- `rand` - Parallel processing capabilities for efficient graph operations.
- `rayon` - Parallel processing capabilities for efficient graph operations.
- `serde` - Support for serialization and deserialization of hypergraphs.
- `wasm` - WebAssembly support for running hypergraph operations in the browser.

### Examples

For more detailed examples, please refer to the [examples directory](https://github.com/FL03/rshyper/blob/main/rshyper/examples).

#### _Example #1:_ Basic Usage

```rust
    extern crate rshyper;

    fn main() -> rshyper::Result<()> {
        // initialize a new instance of a hypergraph
        let mut graph: HashGraph<usize, usize> = HashGraph::new();
        // use the macro to insert nodes into the graph
        rshyper::hypernode! {
            graph {
                let v0;
                let v1 = 2;
                let v2 = 3;
                let v3 = 4;
            }
        }
        // Add some hyperedges
        let e1 = graph.insert_edge(vec![v0, v1, v2])?;
        println!("Added hyperedge {e1}: {:?}", [v0, v1, v2]);

        let e2 = graph.insert_edge(vec![v1, v2, v3])?;
        println!("Added hyperedge {e2}: {:?}", [v1, v2, v3]);

        // Get neighbors of vertex v1
        let neighbors = graph.neighbors(&v1)?;
        println!("Neighbors of {}: {:?}", v1, neighbors);

        // Get degree of vertex v1
        let degree = graph.get_degree_of_node(&v1);
        println!("Degree of {v1}: {degree}");

        // Remove a vertex
        graph.remove_vertex(&v2)?;
        println!("Removed vertex {v2}");

        println!("---------\nFinal graph state: {:?}", graph);
        Ok(())
    }

```

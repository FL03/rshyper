# rshyper

[![crates.io](https://img.shields.io/crates/v/rshyper?style=for-the-badge&logo=rust)](https://crates.io/crates/rshyper)
[![docs.rs](https://img.shields.io/docsrs/rshyper?style=for-the-badge&logo=docs.rs)](https://docs.rs/rshyper)
[![GitHub License](https://img.shields.io/github/license/FL03/rshyper?style=for-the-badge&logo=github)](https://github.com/FL03/rshyper/blob/main/LICENSE)

***

_**Warning: The library is currently in the early stages of development and is not yet ready for production use.**_

`rshyper` is a Rust library designed to provide a hypergraph implementation with a focus on performance and flexibility. It is built to handle complex relationships between data points efficiently, making it suitable for various applications in graph theory, data analysis, and more.

## Background

Before diving in to the technical side of things, let's start by defining several terms commonly used in the definition and implementation of hypergraphs.

- **edge**: here, we consider a hyperedge to specifically define
- **facet**: a facet materializes a hyperedge by associating some weight with the edge.
- **node**: a node is a complete _vertex_ in that it is considered to be weighted.
- **surface**:here, the terms surface and facet are used interchangeably, and they refer to the same concept of a hyperedge with an associated weight.
- **vertex**: a vertex can be understood as a _point_ in space that is used to define edges within a hypergraph.

### Hypergraphs

#### Definition

Formally, a hypergraph is defined as a pair $H = (V, E)$ where:

- $V$ is a set of vertices (or nodes).
- $E$ is a set of hyperedges, where each hyperedge is a subset of $V$ that can contain one or more vertices.

#### Properties

- **Non-binary edges**: Unlike traditional graphs where edges connect exactly two vertices, hyperedges can connect any number of vertices.
- **Flexibility**: Hypergraphs can represent complex relationships and structures that are not easily captured by binary graphs.

## Features

- [x] `hash_graph` - A hash-based hypergraph implementation.

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

## Getting Started

### Prerequisites

Ensure you have the latest version of Rust installed. You can install Rust using [rustup](https://rustup.rs/).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, I always recommend ensuring that rustup is updated to the latest version:

```bash
rustup update
```

And to add the latest nightly toolchain, which is often useful for development:

```bash
rustup toolchain install nightly
```

Additionally, you may wish to install the `cargo-binstall` utility to streamline the installation of Rust binaries:

```bash
cargo install cargo-binstall
```

If necessary, add the `wasm32-*` target(s) if you plan to compile for WebAssembly:

```bash
rustup target add wasm32-unknown-unknown wasm32-p1 wasm32-p2
```

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/rshyper.git -b main --depth 1
```

Then, navigate to the project directory:

```bash
cd rshyper
```

Once you're in the project directory, you can build the project using `cargo`:

```bash
cargo build --workspace --release --all-features
```

Or, if you want to run the tests, you can use:

```bash
cargo test --workspace --release --all-features
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

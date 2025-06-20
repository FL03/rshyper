# rshyper

[![crates.io](https://img.shields.io/crates/v/rshyper?style=for-the-badge&logo=rust)](https://crates.io/crates/rshyper)
[![docs.rs](https://img.shields.io/docsrs/rshyper?style=for-the-badge&logo=docs.rs)](https://docs.rs/rshyper)
[![GitHub License](https://img.shields.io/github/license/FL03/rshyper?style=for-the-badge&logo=github)](https://github.com/FL03/rshyper/blob/main/LICENSE)

***

_**Warning: The library is still in development so prepare for a shifting api...**_

`rshyper` is a Rust library designed to provide a hypergraph implementation with a focus on performance and flexibility. It is built to handle complex relationships between data points efficiently, making it suitable for various applications in graph theory, data analysis, and more.

## Table of Contents

- [Usage](#usage)
  - [Features](#features)
  - [Examples](#examples)
- [Getting Started](#getting-started)
- [Contributing](#contributing)
- [License](#license)
- [Security](#security)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.rshyper]
features = [
    "hyper_map",
    "macros",
]
version = "0.1.x"
```

### Features

The `rshyper` library provides several features to enhance and isolate its functionality:

- `hyper_map` - A map-based hypergraph implementation.
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
        let mut graph: HyperMap<usize, usize> = HyperMap::new();
        // use the macro to insert nodes into the graph
        rshyper::hypergraph! {
            graph {
                nodes: {
                  let v0;
                  let v1 = 2;
                  let v2 = 3;
                  let v3 = 4;
                };
                esges: {
                  let e0: [v0, v1, v2];
                  let e1: [v0, v2, v3];
                };
            }
        }
        // view the initial properties
        println!("Initial graph state: {:?}", graph);
        // Get neighbors of vertex v1
        let neighbors = graph.neighbors(&v1)?;
        println!("Neighbors of {}: {:?}", v1, neighbors);

        // Get degree of vertex v1
        let degree = graph.get_degree_of_node(&v1);
        println!("Degree of {v1}: {degree}");

        // Remove a vertex
        graph.remove_vertex(&v2)?;
        println!("Removed vertex {v2}");

        println!("*********\nFinal graph state:\n*********\n{:?}", graph);
        Ok(())
    }

```

## Getting Started

View the [QUICKSTART](https://github.com/FL03/rshyper/blob/main/QUICKSTART.md) guide for a detailed introduction to using `rshyper`, including how to set up your environment, basic operations, and examples of hypergraph manipulation.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

This project is licensed under the Apache-2.0 License - see the [LICENSE](https://github.com/FL03/rshyper/blob/main/LICENSE) file for details.

## Security

View the [Security Policy](https://github.com/FL03/rshyper/blob/main/SECURITY.md) for details on how to report security vulnerabilities.

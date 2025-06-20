# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.rshyper]
features = [
    "hash_graph",
    "macros",
]
version = "0.1.x"
```

## Features

The `rshyper` library provides several features to enhance and isolate its functionality:

- `hash_graph` - A hash-based hypergraph implementation.
- `macros` - A set of macros to simplify hypergraph creation and manipulation.

### _Dependency related features_

- `rand` - Parallel processing capabilities for efficient graph operations.
- `rayon` - Parallel processing capabilities for efficient graph operations.
- `serde` - Support for serialization and deserialization of hypergraphs.
- `wasm` - WebAssembly support for running hypergraph operations in the browser.

## Examples

For more detailed examples, please refer to the [examples directory](https://github.com/FL03/rshyper/blob/main/rshyper/examples).

### _Example #1: Basic Usage_

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

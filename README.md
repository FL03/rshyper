# rshyper

[![crates.io](https://img.shields.io/crates/v/rshyper.svg)](https://crates.io/crates/rshyper)
[![docs.rs](https://docs.rs/rshyper/badge.svg)](https://docs.rs/rshyper)
[![license](https://img.shields.io/crates/l/rshyper.svg)](https://crates.io/crates/rshyper)

[![clippy](https://github.com//rshyper/actions/workflows/clippy.yml/badge.svg)](https://github.com//rshyper/actions/workflows/clippy.yml)
[![rust](https://github.com//rshyper/actions/workflows/rust.yml/badge.svg)](https://github.com//rshyper/actions/workflows/rust.yml)

***

_**The library is currently in the early stages of development and is not yet ready for production use.**_

This crate focuses on hypergraphs

## Features

- [x] Feature 1

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
version = "0.1.0"
```

### Examples

#### _Basic Usage_

```rust
    extern crate rshyper;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt::init();
        tracing::info!("Welcome to {name}", name = rshyper);


        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

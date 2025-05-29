---
title: Quickstart
description: A detailed guide on how to get started with the rshyper project.
---

## Prerequisites

Listed below are the tools and dependencies required to build and run the project:

- [Docker](https://docs.docker.com/get-docker/)
- [Rust](https://www.rust-lang.org/tools/install)

### Additional Utilities

#### `cargo-binstall`

To streamline the installation of additional, cargo-based tooling ensure that [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) is installed. This tool allows you to install Rust binaries easily and quickly.

```bash
cargo install cargo-binstall
```

## Getting Started

Ensure that `rustup` and all installed toolchains are updated:

```bash
rustup update
```

Optionally, instal the `wasm32-*` targets for WebAssembly development:

```bash
rustup target add wasm32-unknown-unknown wasm32-wasip1 wasm32-wasip2
```

### Building from the source

Get started by cloning the repository:

```bash
git clone https://github.com/FL03/rshyper.git --branch main
```

Then, navigate to the project directory:

```bash
cd rshyper
```

#### Native

For native development, you can run the server using cargo:

```bash
cargo run --locked --release --features full --bin pzzld --
```

#### WebAssembly

##### WebAssembly System Interface (wasi)

Build the project using the wasm32 target:

```bash
cargo build --locked --workspace --release --features wasi --target wasm32-wasip2
```

### Docker

You can also build the project using Docker. Start by building the Docker image:

```bash
docker buildx build --platform linux/amd64 -t jo3mccain/rshyper:latest -f ./Dockerfile .
```

Then, run the Docker container:

```bash
docker run -d -it --rm -p 8080:8080 -v $(pwd):/app jo3mccain/rshyper:latest
```

This will start the server and bind it to port 8080 on your host machine. You can access the server by navigating to `http://localhost:8080` in your web browser.

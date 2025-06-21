---
description: A hypergraph framework for Rust
keywords: [hypergraph, rust, graph theory, data analysis]
author: FL03
---

# rshyper

[![crates.io](https://img.shields.io/crates/v/rshyper?style=for-the-badge&logo=rust)](https://crates.io/crates/rshyper)
[![docs.rs](https://img.shields.io/docsrs/rshyper?style=for-the-badge&logo=docs.rs)](https://docs.rs/rshyper)
[![GitHub License](https://img.shields.io/github/license/FL03/rshyper?style=for-the-badge&logo=github)](https://github.com/FL03/rshyper/blob/main/LICENSE)

***

`rshyper` is a Rust library designed to provide a hypergraph implementation with a focus on performance and flexibility. It is built to handle complex relationships between data points efficiently, making it suitable for various applications in graph theory, data analysis, and more.

## Table of Contents

- [Background](#background)
- [Usage](https://fl03.github.io/rshyper/usage)

## Background

Hypergraphs are generalizations of traditional graphs that allow edges to connect any number of vertices, rather than just two. This flexibility makes hypergraphs suitable for modeling complex relationships in various domains, such as social networks, biological systems, and data analysis.

- [Terminology](#terminology)
- [Hypergraphs](#hypergraphs)
  - [Definition](#definition)
  - [Properties](#properties)

### Terminology

Before diving in to the technical side of things, let's start by defining several terms commonly used in the definition and implementation of hypergraphs.

- **edge**: here, we consider a hyperedge to be the generalization of an edge in a traditional graph, allowing it to connect $n$ vertices.
- **link**: a link is used to define the _layout_ of a hyperedge, providing the basic structure of an edge.
- **node**: a node is a complete _vertex_ in that it is considered to be weighted.
- **surface**: a facet, or surface, represents a complete hyperedge equipped with a weight.
- **vertex**: a vertex can be understood as a _point_ in space that is used to define edges within a hypergraph.

### Hypergraphs

A hypergraph is an abstract data-structure that generalizes the concept of a graph (and even that of the simplicial complex). In a hypergraph, edges (called hyperedges) can connect any number of vertices, allowing for more complex relationships than traditional graphs.

#### Definition

Formally, a hypergraph is defined as a pair $H = (V, E)$ where:

- $V$ is a set of vertices (or nodes).
- $E$ is a set of hyperedges, where each hyperedge is a subset of $V$ that can contain one or more vertices such that $E \subseteq 2^V$.

#### Properties

Listed below are some intrinsic properties of hypergraphs:

- **degree:** The degree of a vertex in a hypergraph is the number of hyperedges that contain that vertex.
- **domain:** the domain of a hypergraph `H` is the set of vertices `V` within the edge set `E`.
- **order:** The order of a hypergraph `H` is the number of vertices in `V`.
- **size:** The size of a hypergraph `H` is the number of hyperedges in `E`.

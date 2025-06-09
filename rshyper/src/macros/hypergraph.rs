/*
    appellation: hypergraph <module>
    authors: @FL03
*/
/// the [`hypergraph`] macro works to aide the in the creation of hypergraphs by allowing
/// users to define nodes and edges in a hypergraph in a more declarative way.
///
/// ## Example
///
/// ### Basic Usage
///
/// The `hypergraph` macro allows you to define nodes and edges in a hypergraph
///
/// ```rust
/// use rshyper::HashGraph;
/// // initialize a new undirected hypergraph
/// let mut graph = HashGraph::<usize, usize>::undirected();
/// // use the macro to insert nodes and edges into the graph
/// rshyper::hypergraph! {
///     graph {
///         nodes: {
///             let v0;
///             let v1;
///             let v2 = 10;
///         };
///         edges: {
///             let e0: [v0, v1];
///             let e1: [v0, v1, v2] = 10;
///         };
///     }
/// }
/// ```
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! hypergraph {
    (
        $graph:ident {
            nodes: {$($nodes:tt)*};
            edges: {$($edges:tt)*};
        }

    ) => {
        // insert nodes into the graph
        $crate::hypernode!($graph {$($nodes)*});
        // insert edges into the graph
        $crate::hyperedge!($graph {$($edges)*});
    };
}
/// The `hyperedge` macro streamlines the definition of hyperedges in a hypergraph.
///
/// ## Usage
///
/// The macro requires that you to pass a mutable reference to some hypergraph by first
/// defining the ident of the associated graph. Once declared, hyperedges are defined as `let`
/// statement within a block, where each statement defines a hyperedge by its name and the
/// vertices it connects. Instead of specifying the type of hyperedge, a slice of node indices
/// is used to define the edge consituents. Optionally, a weight may be specified for the
/// hyperedge by appending `= <weight>` immediately after the desired edge statement.
///
/// ```no_run
/// hyperedge! {
///     ${graph} {
///         let ${edge_name}: [${vertex1}, ${vertex2}, ...] $(= ${weight})?;
///     }
/// }
/// ```
///
/// ## Example
///
/// ```rust
/// use rshyper::{HashGraph, Weight};
/// // initialize a new undirected hypergraph
/// let mut graph = HashGraph::<usize, usize>::undirected();
/// // insert some vertices
/// let v0 = graph.add_node(Weight(1)).expect("Failed to insert node");
/// let v1 = graph.add_node(Weight(2)).expect("Failed to insert node");
/// let v2 = graph.add_node(Weight(3)).expect("Failed to insert node");
/// // use the macro to insert edges into the graph
/// rshyper::hyperedge! {
///     graph {
///         let e0: [v0, v1];
///         let e1: [v0, v1, v2] = 10;
///     }
/// }
/// // verify the order of the edges
/// assert_eq!(graph.get_edge_order(&e0), 2);
/// assert_eq!(graph.get_edge_order(&e1), 3);
/// // verify the weights of the edges
/// assert_eq!(graph.get_edge_weight(&e0), &<usize>::default());
/// assert_eq!(graph.get_edge_weight(&e1), &10);
/// ```
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! hyperedge {
    ($src:ident { $(let $edge:ident: [$($var:ident),*] $(= $w:expr)?);* $(;)? }) => {
        $(
            $crate::hyperedge!(@impl let $src.$edge: [$($var),*] $(= $w)?);
        )*
    };
    (@impl let $src:ident.$edge:ident: [$($var:ident),*] $(= $w:expr)?) => {
        $crate::hyperedge!(@new let $src.$edge = [$($var),*] $(=> $w)?);
    };
    (@new let $src:ident.$edge:ident = [$($var:ident),*]) => {
        let $edge = $src.add_edge([$($var),*]).expect("Failed to insert edge");
    };
    (@new let $src:ident.$edge:ident = [$($var:ident),*] => $w:expr) => {
        let $edge = $src.add_surface([$($var),*], $crate::Weight($w)).expect("Failed to insert edge");
    };
}
/// the [`hypernode`] macro streamlines the process of inserting nodes into a hypergraph.
///
/// ## Usage
///
/// The macro requires that you to pass a mutable reference to some hypergraph by first
/// defining the ident of the associated graph. Once declared, hypernodes are defined as `let`
/// statement within a block, where each statement defines the identifier of a node.
/// Optionally, a weight may be specified for the hypernode by appending `= <weight>`.
///
/// ```no_run
/// hypernode! {
///     $source: {
///         let $var:ident $(= $w:expr)?;
///     }
/// }
/// ```
///
/// ### Example
///
/// ```rust
/// use rshyper::HashGraph;
///
/// let mut graph: HashGraph<usize, usize> = HashGraph::new();
/// // use the macro to insert nodes into the graph
/// rshyper::hypernode! {
///     graph {
///         let v0;
///         let v1;
///         let v2;
///         let v3 = 1;
///         let v4 = 2;
///         let v5 = 3;
///     }
///  }
/// ```
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! hypernode {
    ($src:ident { $(let $var:ident $(= $w:expr)?);* $(;)? }) => {
        $($crate::hypernode!(@impl $src.$var $(= $w)?);)*
    };
    (@impl $src:ident.$var:ident $(= $w:expr)?) => {
        $crate::hypernode!(@new $src.$var $(= $w)?);
    };
    (@new $src:ident.$var:ident = $w:expr) => {
        let $var = $src.add_node($crate::Weight($w)).expect("Failed to insert node");
    };
    (@new $src:ident.$var:ident) => {
        let $var = $src.add_vertex().expect("Failed to insert node");
    };
}

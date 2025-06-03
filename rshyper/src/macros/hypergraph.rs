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
#[cfg(feature = "std")]
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

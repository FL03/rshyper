/*
    appellation: hypergraph <module>
    authors: @FL03
*/

#[cfg(feature = "std")]
#[macro_export]
macro_rules! hypergraph {
    (
        [$graph:ident]
        nodes: {
            $(
                let $var:ident$(= $w:expr)?
            );* $(;)?
        }
    ) => {
        // insert nodes into the graph
        $crate::hypernode!($graph { $(let $var $(= $w)?);* });
    };
}
/// the [`hypernode`] macro streamlines the process of inserting nodes into a hypergraph.
///
/// ## Usage
///
/// ```no_run
/// hypernode! {
///     $source: {
///         let $var:ident $(= $w:expr)?;
///     }
/// }
/// ```
///
/// ### Basic Usage
///
/// ```rust
/// use rshyper::HashGraph;
///
/// let mut graph: HashGraph<usize, usize> = HashGraph::new();
/// // use the macro to insert nodes into the graph
/// rshyper::hypernode! {
///     graph {
///         let v0;
///         let v1 = 1;
///         let v2 = 2;
///         let v3 = 3;
///     }
///  }
/// ```
#[macro_export]
macro_rules! hypernode {
    ($src:ident { $(let $var:ident $(= $w:expr)?);* $(;)? }) => {
        $($crate::hypernode!(@impl $src [$var] $(= $w)?);)*
    };
    (@impl $src:ident[$var:ident] $(= $w:expr)?) => {
        $crate::hypernode!(@new $src[$var] $(= $w)?);
    };
    (@new $src:ident[$var:ident] = $w:expr) => {
        let $var = $src.insert_node($w);
    };
    (@new $src:ident[$var:ident]) => {
        let $var = $src.insert_node_default();
    };
}

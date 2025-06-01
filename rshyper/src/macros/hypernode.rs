/*
    appellation: hypernode <module>
    authors: @FL03
*/

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
///         let v1;
///         let v2;
///         let v3 = 1;
///         let v4 = 2;
///         let v5 = 3;
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
        let $var = $src.insert_vertex();
    };
}

/*
    appellation: hypergraph <module>
    authors: @FL03
*/
/// the [`hypergraph`] macro works to aide the in the creation of hypergraphs by allowing
/// users to define nodes in a more structured way.
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

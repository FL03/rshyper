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
        $graph:ident {
            nodes {$($nodes:tt)*};
            edges {$($edges:tt)*};
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
    ($src:ident { $(let $edge:ident = [$($var:ident),*] $(=> $w:expr)?);* $(;)? }) => {
        $(
            $crate::hyperedge!(@impl let $src.$edge = [$($var),*] $(=> $w)?);
        )*
    };
    (@impl let $src:ident.$edge:ident = [$($var:ident),*] $(=> $w:expr)?) => {
        $crate::hyperedge!(@new let $src.$edge = [$($var),*] $(=> $w)?);
    };
    (@new let $src:ident.$edge:ident = [$($var:ident),*]) => {
        let $edge = $src.add_edge([$($var),*]).expect("Failed to insert edge");
    };
    (@new let $src:ident.$edge:ident = [$($var:ident),*] => $w:expr) => {
        let $edge = $src.add_edge_with_weight([$($var),*], $w).expect("Failed to insert edge");
    };
}



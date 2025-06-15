/*
    appellation: rshyper-macros <library>
    authors: @FL03
*/
//! # rshyper-macros
//!
//! this crate defines various procedural macros for the `rshyper` crate working to stramline
//! the creation and manipulation of hypergraphs.
//!

pub(crate) mod ast;
pub(crate) mod attr;

use self::ast::GraphAst;
use proc_macro::TokenStream;
use quote::quote;

#[doc(hidden)]
#[proc_macro]
/// this macro is used to create new nodes and edges in a hypergraph
pub fn hygraph(input: TokenStream) -> TokenStream {
    let GraphAst { graph, nodes, edges } = syn::parse_macro_input!(input as GraphAst);

    let node_stmts = nodes.iter().map(|n| {
        quote! {
            let #n = #graph.add_vertex().unwrap();
        }
    });
    let edge_stmts = edges.iter().map(|(e, vs)| {
        quote! {
            let #e = #graph.add_edge([#(#vs),*]).unwrap();
        }
    });
    // generate the output code
    let out = quote! {
        #(#node_stmts)*
        #(#edge_stmts)*
    };
    // convert the output into the correct TokenStream
    TokenStream::from(out)
}

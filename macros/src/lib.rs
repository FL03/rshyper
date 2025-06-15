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
pub(crate) mod impls;

use self::ast::GraphAst;
use proc_macro::TokenStream;

#[doc(hidden)]
#[proc_macro]
/// this macro is used to create new nodes and edges in a hypergraph
pub fn hygraph(input: TokenStream) -> TokenStream {
    let GraphAst {
        graph,
        nodes,
        edges,
        ..
    } = syn::parse_macro_input!(input as GraphAst);

    let node_stmts = nodes.iter().map(|node| impls::handle_node(&graph, node));
    let edge_stmts = edges.iter().map(|edge| impls::handle_edge(&graph, edge));
    // generate the output code
    let out = quote::quote! {
        #(#node_stmts)*
        #(#edge_stmts)*
    };
    // convert the output into the correct TokenStream
    TokenStream::from(out)
}

mod kw {
    syn::custom_keyword!(graph);
    syn::custom_keyword!(nodes);
    syn::custom_keyword!(edges);
}

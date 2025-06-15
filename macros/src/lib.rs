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

use crate::ast::weight;

use self::ast::GraphAst;
use proc_macro::TokenStream;
use quote::quote;

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

    let node_stmts = nodes.iter().map(|ast::NodeAst { key, value, .. }| {
        let ast::VertexAst { key, .. } = key;
        if let Some(weight::WeightAst { expr: value, .. }) = value {
            quote! {
                let #key = #graph.add_node(#value.into()).expect("failed to add node");
            }
        } else {
            quote! {
                let #key = #graph.add_vertex().expect("failed to add node");
            }
        }
    });
    let edge_stmts = edges.iter().map(|ast::EdgeAst { key, nodes, weight, .. }| {
        if let Some(weight::WeightAst { expr: value, .. }) = weight {
            quote! {
                let #key = #graph.add_surface(#nodes, #value.into()).expect("failed to add edge");
            }
        } else {
            quote! {
                let #key = #graph.add_edge(#nodes).expect("failed to add edge");
            }
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

mod kw {
    syn::custom_keyword!(graph);
    syn::custom_keyword!(nodes);
    syn::custom_keyword!(edges);
}

/*
    appellation: node <module>
    authors: @FL03
*/
use crate::ast::{EdgeAst, NodeAst, WeightAst};
use quote::quote;
use syn::Ident;

pub fn handle_edge(g: &Ident, edge: &EdgeAst) -> proc_macro2::TokenStream {
    let EdgeAst {
        key, nodes, weight, ..
    } = edge;
    if let Some(WeightAst { expr: value, .. }) = weight {
        quote! {
            let #key = #g.add_surface(#nodes, #value.into()).expect("failed to add edge");
        }
    } else {
        quote! {
            let #key = #g.add_edge(#nodes).expect("failed to add edge");
        }
    }
}

pub fn handle_node(g: &Ident, node: &NodeAst) -> proc_macro2::TokenStream {
    let NodeAst { key, value, .. } = node;
    if let Some(WeightAst { expr: value, .. }) = value {
        quote! {
            #key #g.add_node(#value.into()).expect("failed to add node");
        }
    } else {
        quote! {
            #key #g.add_vertex().expect("failed to add node");
        }
    }
}

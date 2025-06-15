/*
    appellation: graph <ast>
    authors: @FL03
*/
use super::{EdgeAst, NodeAst};
use crate::kw;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, Token, braced};

/// The [`GraphAst`] struct represents the abstract syntax tree (AST) for a hypergraph macro.
/// Overall, it is supposed to feel similar to
///
/// ```ignore
/// hypergraph! {
///     graph {
///         nodes: {
///             let v0;
///             let v1 = 90;
///             let v2 = 100;
///         };
///         edges: {
///             let e0: [v0, v1];
///             let e1: [v0, v1, v2] = 50;
///         };
///     }
/// }
/// ```
pub struct GraphAst {
    pub graph: Ident,
    pub nodes: Punctuated<NodeAst, Token![;]>,
    pub edges: Punctuated<EdgeAst, Token![;]>,
}

impl Parse for GraphAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let graph: Ident = input.parse()?;
        let content; // the graph {} outer brace
        braced!(content in input);

        let mut nodes = Punctuated::new();
        let mut edges = Punctuated::new();
        while !content.is_empty() {
            if input.peek(kw::nodes) {
                input.parse::<kw::nodes>()?;
                input.parse::<Token![:]>()?;
                let node_content;
                braced!(node_content in input);
                nodes = node_content.parse_terminated(NodeAst::parse, Token![;])?;
                input.parse::<Token![;]>()?;
            }
            if input.peek(kw::edges) {
                input.parse::<kw::edges>()?;
                input.parse::<Token![:]>()?;
                let edge_content;
                braced!(edge_content in input);
                edges = edge_content.parse_terminated(EdgeAst::parse, Token![;])?;
                input.parse::<Token![;]>()?;
            }
        }
        Ok(GraphAst {
            graph,
            nodes,
            edges,
        })
    }
}

/*
    appellation: graph <ast>
    authors: @FL03
*/
use syn::{Ident, Token, braced};
use syn::parse::{Parse, ParseStream};

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
    pub nodes: Vec<Ident>,
    pub edges: Vec<(Ident, Vec<Ident>)>,
}

impl Parse for GraphAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content; // the graph {} outer brace
        let graph: Ident = input.parse()?;
        braced!(content in input);
        input.parse::<Ident>()?; // nodes
        input.parse::<Token![:]>()?;
        let node_content; // the nodes: {} brace
        braced!(node_content in input);
        let nodes = node_content.parse_terminated(Ident::parse, Token![,])?.into_iter().collect();
        input.parse::<Token![;]>()?;
        input.parse::<Ident>()?; // edges
        input.parse::<Token![:]>()?;
        let edge_content; // the edges: {} brace
        braced!(edge_content in input);
        let mut edges = Vec::new();
        while !edge_content.is_empty() {
            let e_name: Ident = edge_content.parse()?;
            edge_content.parse::<Token![:]>()?;
            let inner;
            syn::bracketed!(inner in content);
            let edge_nodes = inner.parse_terminated(Ident::parse, Token![,])?.into_iter().collect();
            edges.push((e_name, edge_nodes));
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }
        input.parse::<Token![;]>()?;
        Ok(GraphAst { graph, nodes, edges })
    }
}

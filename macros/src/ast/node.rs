/*
    appellation: node <ast>
    authors: @FL03
*/
use super::WeightAst;
use syn::{Attribute, Ident};
use syn::parse::{Parse, ParseStream};
use syn::token;

#[allow(dead_code)]
pub struct VertexAst {
    pub let_token: token::Let,
    pub key: Ident,
}

/// The [`NodeAst`] struct represents the abstract syntax tree (AST) for a edges within the
/// hypergraph macro.
///
/// ```ignore
/// edges: {
///     let e0: [v0, v1];
///     let e1: [v0, v1, v2] = 50;
/// };
/// ```
#[allow(dead_code)]
pub struct NodeAst {
    pub attrs: Vec<Attribute>,
    pub key: VertexAst,
    pub value: Option<WeightAst>,
}

impl Parse for VertexAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let let_token = input.parse()?;
        let key = input.parse()?;

        Ok(Self { let_token, key })
    }
}

impl Parse for NodeAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        // parse the vertex definition
        let vertex = input.parse::<VertexAst>()?;
        // determine if there is a weight associated with the node
        let value = if input.peek(token::Eq) {
            Some(input.parse::<WeightAst>()?)
        } else {
            None
        };
        Ok(Self {
            attrs,
            key: vertex,
            value,
        })
    }
}

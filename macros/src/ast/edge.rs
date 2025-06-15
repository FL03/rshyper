/*
    appellation: graph <ast>
    authors: @FL03
*/
use super::WeightAst;
use syn::{token, Attribute, Ident, Token, bracketed};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

/// The [`EdgeAst`] struct represents the abstract syntax tree (AST) for a edges within the
/// hypergraph macro.
///
/// ```ignore
/// edges: {
///     let e0: [v0, v1];
///     let e1: [v0, v1, v2] = 50;
/// };
/// ```
#[allow(dead_code)]
pub struct EdgeAst {
    pub attrs: Vec<Attribute>,
    pub let_token: token::Let,
    pub key: Ident,
    pub colon_token: Token![:],
    pub bracket: token::Bracket,
    pub nodes: Punctuated<Ident, Token![,]>,
    pub weight: Option<WeightAst>,
}


impl Parse for EdgeAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let let_token = input.parse()?;
        let key = input.parse()?;
        let colon_token = input.parse()?;
        let nodes;
        let node_bracket = bracketed!(nodes in input);
        let nodes = nodes.parse_terminated(Ident::parse, Token![,])?;

        let weight = if input.peek(token::Eq) {
            Some(input.parse::<WeightAst>()?)
        } else {
            None
        };

        Ok(EdgeAst {
            attrs,
            let_token,
            key,
            colon_token,
            bracket: node_bracket,
            nodes,
            weight,
        })
    }
}

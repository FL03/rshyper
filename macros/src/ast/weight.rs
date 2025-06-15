/*
    appellation: node <ast>
    authors: @FL03
*/
use syn::parse::{Parse, ParseStream};
use syn::{Expr, token};

#[allow(dead_code)]
pub struct WeightAst {
    pub eq_token: token::Eq,
    pub expr: Box<Expr>,
}

impl Parse for WeightAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let eq_token = input.parse()?;
        let expr = input.parse()?;

        Ok(Self {
            eq_token,
            expr: Box::new(expr),
        })
    }
}

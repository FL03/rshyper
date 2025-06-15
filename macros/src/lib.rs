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

use proc_macro::TokenStream;

#[doc(hidden)]
#[proc_macro]
/// this macro is used to create new nodes and edges in a hypergraph
pub fn nodes(input: TokenStream) -> proc_macro::TokenStream {
    // pass the input to the `ast` module for processing
    input
}

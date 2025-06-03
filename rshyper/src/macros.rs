/*
    appellation: macros <module>
    authors: @FL03
*/
//! this module implements various macros used throughout the rshyper framework
//!
//!
#[cfg(feature = "macros")]
#[macro_use]
pub mod hypergraph;
#[cfg(feature = "macros")]
#[macro_use]
pub mod hypernode;
#[macro_use]
pub mod seal;

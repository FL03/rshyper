/*
    Appellation: topo <module>
    Contrib: @FL03
*/
//! This module implements the foundation for the substrate, integrating the harmonically
//! inspired topological entities and operations.
//!
//!
#[doc(inline)]
pub use self::{tonnetz::Tonnetz, triad::Triad};

pub mod memory;
pub mod motion;
pub mod tonnetz;
pub mod triad;

pub(crate) mod prelude {
    pub use super::tonnetz::*;
    pub use super::triad::*;
}

/*
    Appellation: topo <module>
    Contrib: @FL03
*/
//! This module implements the foundation for the substrate, integrating the harmonically
//! inspired topological entities and operations.
//!
//!
#[doc(inline)]
pub use self::{plant::Plant, tonnetz::Tonnetz, triad::Triad, types::prelude::*};

pub mod memory;
pub mod motion;
pub mod plant;

pub mod tonnetz;
pub mod triad;

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod class;
    pub mod transform;

    pub(crate) mod prelude {
        pub use super::class::*;
        pub use super::transform::*;
    }
}

pub(crate) mod prelude {
    pub use super::motion::*;
    pub use super::plant::*;
    pub use super::tonnetz::*;
    pub use super::triad::*;
    pub use super::types::*;
}

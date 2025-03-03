/*
    Appellation: mem <module>
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::persistent::TopologicalMemory;

pub mod persistent;

pub(crate) mod prelude {
    pub use super::persistent::*;
}

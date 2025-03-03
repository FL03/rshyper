/*
    Appellation: topo <module>
    Contrib: @FL03
*/
//! This module implements the foundation for the substrate, integrating the harmonically
//! inspired topological entities and operations.
//!
//!
#[doc(inline)]
pub use self::{tonnetz::Tonnetz, triad::Triad, types::prelude::*, utils::prelude::*};

pub mod motion;

pub mod tonnetz;
pub mod triad;

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod class;
    pub mod factors;
    pub mod lpr;

    pub(crate) mod prelude {
        pub use super::class::*;
        pub use super::factors::*;
        pub use super::lpr::*;
    }
}

pub mod utils {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod paths;

    pub(crate) mod prelude {
        pub use super::paths::*;
    }
}

pub(crate) mod prelude {
    pub use super::motion::*;
    pub use super::tonnetz::*;
    pub use super::triad::*;
    pub use super::types::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_leading() {
        // c-major
        let triad = Triad::from_root(0, TriadClass::Major);
        // e-minor
        let next = triad.transform(LPR::Leading);
        assert_eq!(next, Triad::from_root(4, TriadClass::Minor));
        // invert
        assert_eq!(triad, next.leading());
    }

    #[test]
    fn test_parallel() {
        // c-major
        let triad = Triad::from_root(0, TriadClass::Major);
        // c-minor
        let next = triad.transform(LPR::Parallel);
        assert_eq!(next, Triad::from_root(0, TriadClass::Minor));
        // invert
        assert_eq!(triad, next.parallel());
    }

    #[test]
    fn test_relative() {
        // c-major
        let triad = Triad::from_root(0, TriadClass::Major);
        // a-minor
        let next = triad.transform(LPR::Relative);
        assert_eq!(next, Triad::from_root(9, TriadClass::Minor));
        // invert
        assert_eq!(triad, next.relative());
    }
}

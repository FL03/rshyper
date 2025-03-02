/*
    Appellation: rshyper <library>
    Contrib: Joe McCain III <jo3mccain@icloud.com>
*/
//! # proton-substrate
//!
//! This is it my dude.
#![crate_name = "proton"]
#![crate_type = "lib"]

#[allow(unused_imports)]
#[doc(inline)]
pub use self::{
    error::*, models::prelude::*, ops::prelude::*, topo::prelude::*, traits::prelude::*,
    types::prelude::*, utils::prelude::*,
};

#[macro_use]
pub(crate) mod macros;

pub mod error;
pub mod topo;

pub mod models {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod vnode;
    pub mod wolfram;

    pub(crate) mod prelude {
        pub use super::vnode::*;
        pub use super::wolfram::*;
    }
}

pub mod ops {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod num;

    pub(crate) mod prelude {
        pub use super::num::*;
    }
}

#[allow(unused_imports)]
pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub(crate) mod prelude {}
}

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod turing;

    pub(crate) mod prelude {
        pub use super::turing::*;
    }
}

pub mod utils {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod modulus;

    pub(crate) mod prelude {
        pub use super::modulus::*;
    }
}

#[allow(unused_imports)]
pub mod prelude {
    pub use crate::error::*;
    pub use crate::models::prelude::*;
    pub use crate::ops::prelude::*;
    pub use crate::topo::prelude::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
    pub use crate::utils::prelude::*;
}

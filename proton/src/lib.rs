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
    error::*, ops::prelude::*, traits::prelude::*, types::prelude::*, utils::prelude::*, wolfram::*,
};

pub mod error;
pub mod wolfram;

#[allow(unused_imports)]
pub mod ops {
    #[doc(inline)]
    pub use self::prelude::*;

    pub(crate) mod prelude {}
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

    pub mod music;
    pub mod turing;

    pub(crate) mod prelude {
        pub use super::music::*;
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
    pub use crate::ops::prelude::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
    pub use crate::utils::prelude::*;
    pub use crate::wolfram::*;
}

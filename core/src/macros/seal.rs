/*
    Appellation: seal <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! The public parts of this private module are used to create traits
//! that cannot be implemented outside of our own crate.  This way we
//! can feel free to extend those traits without worrying about it
//! being a breaking change for other implementations.

/// If this type is pub but not publicly reachable, third parties
/// can't name it and can't implement traits using it.
#[allow(dead_code)]
pub struct Seal;

#[allow(unused_macros)]
macro_rules! private {
    () => {
        /// This trait is private to implement; this method exists to make it
        /// impossible to implement outside the crate.
        #[doc(hidden)]
        fn __private__(&self) -> $crate::macros::seal::Seal;
    };
}
#[allow(unused_macros)]
macro_rules! seal {
    () => {
        fn __private__(&self) -> $crate::macros::seal::Seal {
            $crate::macros::seal::Seal
        }
    };
}

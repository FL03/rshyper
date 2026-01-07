/*
    appellation: kinds <module>
    authors: @FL03
*/

#[doc(hidden)]
/// [`WeightMode`] is a trait used to denote marker types that define that state of a weight,
/// meaning that the type is either weighted or unweighted. This trait is sealed to prevent further
/// implementations.
pub trait WeightMode: 'static + Send + Sync + core::fmt::Debug + core::fmt::Display {
    // seals the trait to prevent further implementations
    private!();
}

macro_rules! impl_weight_mode {
    (@impl $(#[doc $($doc:tt)*])? $vis:vis $i:ident $kind:ident) => {
        // create the implementation for the kind
        impl_weight_mode!(@branch $(#[doc $($doc)*])? $vis $i $kind);
        // implement the necessary traits for the kind
        impl_weight_mode!(@kind $kind);
        // stringify the kind and implement Display
        impl ::core::fmt::Display for $kind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                // stringify the ident of the kind
                let tag = stringify!($kind);
                // write the tag in lowercase
                write!(f, "{}", tag.to_lowercase())
            }
        }
    };
    (@branch $(#[doc $($doc:tt)*])? $vis:vis enum $kind:ident) => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde_derive::Deserialize, serde_derive::Serialize),
        )]
        #[repr(transparent)]
        $vis enum $kind {};
    };
    (@branch $(#[doc $($doc:tt)*])? $vis:vis struct $kind:ident) => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde_derive::Deserialize, serde_derive::Serialize),
        )]
        #[repr(transparent)]
        $vis struct $kind;
    };
    (@kind $kind:ident) => {
        unsafe impl ::core::marker::Send for $kind {}

        unsafe impl ::core::marker::Sync for $kind {}

        impl $crate::weight::WeightMode for $kind {
            seal!();
        }
    };
    ($($(#[doc $($doc:tt)*])? $vis:vis $i:ident $kind:ident);* $(;)?) => {
        $(
            impl_weight_mode!(@impl $(#[doc $($doc)*])? $vis $i $kind);
        )*
    };
}

impl_weight_mode! {
    pub struct UnWeighted;
    pub struct IsWeighted;
}

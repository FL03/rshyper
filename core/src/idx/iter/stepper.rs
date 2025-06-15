/*
    appellation: stepper <module>
    authors: @FL03
*/
//! this module implements a stepper iterator for indices, replacing and returning the current
//! value with the next using some means (literal, functional, etc.)
//!
use crate::idx::IndexBase;

#[allow(dead_code)]
pub struct Stepper<'a, I, K, F>
where
    F: FnMut(&I) -> I,
{
    pub(crate) curr: IndexBase<I, K>,
    pub(crate) step_fn: F,
    _lt: core::marker::PhantomData<&'a I>,
}

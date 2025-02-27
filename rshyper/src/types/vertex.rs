/*
    Appellation: node <module>
    Contrib: @FL03
*/
use crate::Index;

pub trait Weighted {
    type Data;

    fn weight(&self) -> &Self::Data;
}
pub struct Vertex<T, Idx = u32> {
    pub(crate) index: Index<Idx>,
    pub(crate) weight: core::mem::MaybeUninit<T>,
}

impl<T, Idx> Vertex<T, Idx> {
    pub fn new(index: Index<Idx>) -> Self {
        Self {
            index,
            weight: core::mem::MaybeUninit::uninit(),
        }
    }
    /// returns a pointer to the inner value
    pub const fn as_ptr(&self) -> *const T {
        self.weight.as_ptr()
    }

    /// returns a mutable pointer to the inner value
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.weight.as_mut_ptr()
    }

    pub const fn index(&self) -> &Index<Idx> {
        &self.index
    }
}

impl<T, Idx> Weighted for Vertex<T, Idx> {
    type Data = T;

    fn weight(&self) -> &Self::Data {
        if self.weight.as_ptr().is_null() {
            panic!("Vertex weight is not initialized");
        }
        unsafe { self.weight.assume_init_ref() }
    }
}

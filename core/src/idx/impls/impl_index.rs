/*
    appellation: impl_index <module>
    authors: @FL03
*/
use crate::AddStep;
use crate::idx::{IndexBase, RawIndex};

impl<T, K> Default for IndexBase<T, K>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            value: T::default(),
            _type: core::marker::PhantomData::<K>,
        }
    }
}

impl<T, K> AsRef<T> for IndexBase<T, K>
where
    T: RawIndex,
{
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T, K> AsMut<T> for IndexBase<T, K>
where
    T: RawIndex,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T, K> core::borrow::Borrow<T> for IndexBase<T, K>
where
    T: RawIndex,
{
    fn borrow(&self) -> &T {
        &self.value
    }
}
impl<T, K> core::borrow::BorrowMut<T> for IndexBase<T, K>
where
    T: RawIndex,
{
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T, K> From<T> for IndexBase<T, K>
where
    T: RawIndex,
{
    fn from(index: T) -> Self {
        Self::new(index)
    }
}

impl<T, K> Iterator for IndexBase<T, K>
where
    T: RawIndex + AddStep<Output = T>,
{
    type Item = IndexBase<T, K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.step().ok()
    }
}

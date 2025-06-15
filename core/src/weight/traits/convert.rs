/*
    appellation: convert <module>
    authors: @FL03
*/
use crate::weight::Weight;

/// a trait for converting a type into a valid [`Weight`]
pub trait AsWeight<T> {
    fn as_weight(&self) -> Weight<T>;
}
/// a trait for converting a type into a valid [`Weight`]
pub trait IntoWeight<T> {
    fn into_weight(self) -> Weight<T>;
}

/*
 ************* Implementations *************
*/

impl<T> IntoWeight<T> for T {
    fn into_weight(self) -> Weight<T> {
        Weight::new(self)
    }
}

impl<T> AsWeight<T> for T
where
    T: Clone + IntoWeight<T>,
{
    fn as_weight(&self) -> Weight<T> {
        self.clone().into_weight()
    }
}

/*
    appellation: impl_weight_deprecated <module>
    authors: @FL03
*/
use crate::weight::Weight;

impl<T> Weight<T> {
    #[deprecated(
        note = "use `into_inner` instead, this method will be removed in the next major version",
        since = "0.1.2"
    )]
    #[inline]
    pub fn value(self) -> T {
        self.into_inner()
    }
}

/*
    appellation: weight <module>
    authors: @FL03
*/

/// The [`Weight`] type is a wrapper around a generic type `T` that provides additional
/// functionality for working with weights in a graph context.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent, rename_all = "lowercase")
)]
#[repr(transparent)]
pub struct Weight<T>(pub T);

impl<T> Weight<T> {
    /// returns a new instance of the [`Weight`] with the default value of the inner type.
    pub fn new() -> Self
    where
        T: Default,
    {
        Self(Default::default())
    }
    /// returns a new instance of the [`Weight`] created from the given value.
    pub fn from_value(value: T) -> Self {
        Self(value)
    }
    /// consumes the current instance to return the inner value
    pub fn value(self) -> T {
        self.0
    }
    #[deprecated(
        note = "use `value` instead, this method will be removed in the next major version",
        since = "0.0.8"
    )]
    pub fn into_inner(self) -> T {
        self.0
    }
    /// returns an immutable reference to the inner value.
    pub const fn get(&self) -> &T {
        &self.0
    }
    /// returns a mutable reference to the inner value.
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    /// applies the provided function onto the inner value and returns a new [`Weight`] with
    /// the result.
    pub fn map<U, F>(self, f: F) -> Weight<U>
    where
        F: FnOnce(T) -> U,
    {
        Weight(f(self.0))
    }
    /// [`replace`](core::mem::replace) the inner value and return the previous value.
    pub const fn replace(&mut self, value: T) -> T {
        core::mem::replace(self.get_mut(), value)
    }
    /// updates the inner value with the provided value and returns a mutable reference to the
    /// current instance.
    pub fn set(&mut self, value: T) -> &mut Self {
        *self.get_mut() = value;
        self
    }
    /// [`swap`](core::mem::swap) the inner value with another [`Weight`].
    pub const fn swap(&mut self, other: &mut Self) {
        core::mem::swap(self.get_mut(), other.get_mut());
    }
    /// [`take`](core::mem::take) the inner value, leaving the logical default in its place.
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(self.get_mut())
    }
    /// returns a new [`Weight`] with the inner value cloned.
    pub fn cloned(&self) -> Weight<T>
    where
        T: Clone,
    {
        Weight(self.get().clone())
    }
    /// copies the inner value and returns a new weight with the copied value
    pub fn copied(&self) -> Weight<T>
    where
        T: Copy,
    {
        Weight(*self.get())
    }
    /// returns a constant pointer to the inner value; see [`core::ptr::addr_of!`] for more
    /// information
    pub fn as_ptr(&self) -> *const T {
        core::ptr::addr_of!(self.0)
    }
    /// returns a mutable pointer to the inner value; see [`core::ptr::addr_of_mut!`] for more
    /// information
    pub fn as_mut_ptr(&mut self) -> *mut T {
        core::ptr::addr_of_mut!(self.0)
    }
    /// returns a _view_ of the weight whose inner value is a reference to the original.
    pub const fn view(&self) -> Weight<&T> {
        Weight(self.get())
    }
    /// returns a _view_ of the weight whose inner value is a mutable reference to the original
    pub const fn view_mut(&mut self) -> Weight<&mut T> {
        Weight(self.get_mut())
    }
    /// consumes the current instance to create another with the given value
    pub fn with<U>(self, value: U) -> Weight<U> {
        Weight(value)
    }
    #[deprecated(
        note = "use `view` instead, this method will be removed in the next major version",
        since = "0.0.8"
    )]
    pub const fn as_view(&self) -> Weight<&T> {
        Weight(self.get())
    }
    #[deprecated(
        note = "use `view_mut` instead, this method will be removed in the next major version",
        since = "0.0.8"
    )]
    pub const fn as_view_mut(&mut self) -> Weight<&mut T> {
        Weight(self.get_mut())
    }
}

scsys::fmt_wrapper! {
    Weight<T>(
        Binary,
        Debug,
        Display,
        LowerExp,
        LowerHex,
        UpperExp,
        UpperHex,
        Octal,
        Pointer,
    )
}

impl<T> AsRef<T> for Weight<T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T> AsMut<T> for Weight<T> {
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::borrow::Borrow<T> for Weight<T> {
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T> core::borrow::BorrowMut<T> for Weight<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::ops::Deref for Weight<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> core::ops::DerefMut for Weight<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T> From<T> for Weight<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> PartialEq<T> for Weight<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.get() == other
    }
}

impl<T> PartialOrd<T> for Weight<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

macro_rules! impl_wrapper_binary_op {
    ($s:ident::<[$($op:ident.$call:ident),* $(,)?]>) => {
        $(
            impl_wrapper_binary_op!(@impl $s::$op.$call);
            impl_wrapper_binary_op!(@mut $s::$op.$call);
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident) => {
        impl<A, B, C> ::core::ops::$op<$s<B>> for $s<A>
        where
            A: ::core::ops::$op<B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<$s<B>> for &'a $s<A>
        where
            A: Copy + ::core::ops::$op<B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a $s<B>> for &'a $s<A>
        where
            A: Copy + ::core::ops::$op<B, Output = C>,
            B: Copy
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a $s<B>> for $s<A>
        where
            A: ::core::ops::$op<B, Output = C>,
            B: Copy
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<$s<B>> for &'a mut $s<A>
        where
            A: Copy + ::core::ops::$op<B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a mut $s<B>> for $s<A>
        where
            A: ::core::ops::$op<B, Output = C>,
            B: Copy
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a mut $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a mut $s<B>> for &'a mut $s<A>
        where
            A: Copy + ::core::ops::$op<B, Output = C>,
            B: Copy
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a mut $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.0, rhs.0))
            }
        }
    };
    (@mut $s:ident::$op:ident.$call:ident) => {
        paste::paste! {
            impl_binary_op_mut!(@impl $s::[<$op Assign>].[<$call _assign>]);
        }
    };
}

macro_rules! impl_binary_op_mut {
    ($s:ident::<[$($op:ident.$call:ident),* $(,)?]>) => {
        $(
            impl_wrapper_binary_op!(@impl $s::$op.$call);
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident) => {
        impl<A, B> ::core::ops::$op<$s<B>> for &mut $s<A>
        where
            A: ::core::ops::$op<B>,
        {

            fn $call(&mut self, rhs: $s<B>) {
                core::ops::$op::$call(&mut self.0, rhs.0)
            }
        }
    };
}

macro_rules! impl_unary_op {
    ($s:ident::<[$($op:ident.$call:ident),* $(,)?]>) => {
        $(
            impl_unary_op!(@impl $s::$op.$call);
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident) => {
        impl<A> ::core::ops::$op for &mut $s<A>
        where
            A: Clone + ::core::ops::$op,
        {
            type Output = $s<A::Output>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(self.0.clone()))
            }
        }

        impl<'a, A> ::core::ops::$op for &mut &'a $s<A>
        where
            A: Clone + ::core::ops::$op,
        {
            type Output = $s<A::Output>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(self.0.clone()))
            }
        }

        impl<'a, A> ::core::ops::$op for &mut &'a mut $s<A>
        where
            A: Clone + ::core::ops::$op,
        {
            type Output = $s<A::Output>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(self.0.clone()))
            }
        }
    };
}

impl_wrapper_binary_op! {
    Weight::<[
        Add.add,
        Sub.sub,
        Mul.mul,
        Div.div,
        Rem.rem,
        BitAnd.bitand,
        BitOr.bitor,
        BitXor.bitxor,
        Shl.shl,
        Shr.shr
    ]>
}

impl_unary_op! {
    Weight::<[
        Neg.neg,
        Not.not
    ]>
}

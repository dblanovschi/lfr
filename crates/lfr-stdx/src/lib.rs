mod also
{
    //! Kotlin-like `also` expressions.

    /// Trait to implement `also` on all types
    pub trait Also: Sized
    {
        /// Kotlin-like `also` extension function
        #[inline(always)]
        fn also<F, T>(mut self, f: F) -> Self
            where F: FnOnce(&mut Self) -> T
        {
            let _ = f(&mut self);
            self
        }
    }

    impl<T> Also for T {}
}

mod and_then_do
{
    /// A trait for exposing [`AndThenDo::and_then_do`] on [`Option`] and
    /// [`Result`]
    pub trait AndThenDo
    {
        /// The type of the value
        type VT;
        /// Do an action on the `Ok`/`Some` value and ignore
        /// the returned value
        fn and_then_do<F, U>(self, f: F)
            where F: FnOnce(Self::VT) -> U;
    }

    impl<T, E> AndThenDo for Result<T, E>
    {
        type VT = T;

        #[inline(always)]
        fn and_then_do<F, U>(self, f: F)
            where F: FnOnce(T) -> U
        {
            if let Ok(v) = self {
                let _ = f(v);
            }
        }
    }

    impl<T> AndThenDo for Option<T>
    {
        type VT = T;

        #[inline(always)]
        fn and_then_do<F, U>(self, f: F)
            where F: FnOnce(T) -> U
        {
            if let Some(v) = self {
                let _ = f(v);
            }
        }
    }
}

pub mod clone_to
{
    //! Holds the [`CloneTo`] trait

    /// Trait to define `clone_to*` on all types
    ///
    /// # Examples
    ///
    /// ```
    /// # use lfr_stdx::CloneTo;
    /// let mut a = 0;
    /// assert_eq!(&1, 1_i32.clone_to_ref(&mut a));
    /// assert_eq!(1, a);
    /// ```
    ///
    /// ```
    /// # use lfr_stdx::CloneTo;
    /// let mut a = 0;
    /// assert_eq!(&mut 1, 1_i32.clone_to_ref_mut(&mut a));
    /// assert_eq!(1, a);
    /// ```
    ///
    /// ```
    /// # use lfr_stdx::CloneTo;
    /// let mut a = 0;
    /// assert_eq!(1, 1_i32.clone_to(&mut a));
    /// assert_eq!(1, a);
    /// ```
    pub trait CloneTo: Clone
    {
        /// Clones other from self and returns self; reference variant.
        #[inline(always)]
        fn clone_to_ref(&self, other: &mut Self) -> &Self
        {
            other.clone_from(self);
            self
        }

        /// Clones other from self and returns self; mutable reference variant.
        #[inline]
        fn clone_to_ref_mut(&mut self, other: &mut Self) -> &mut Self
        {
            other.clone_from(self);
            self
        }

        /// Clones other from self and returns self; owned variant.
        #[inline(always)]
        fn clone_to(self, other: &mut Self) -> Self
        {
            other.clone_from(&self);
            self
        }
    }

    impl<T: Clone> CloneTo for T {}
}

mod copied_val
{
    /// Copies the value and returns it, so no need to go back and use `*`.
    pub trait CopiedVal: Sized + Copy
    {
        #[inline(always)]
        fn copied_val(&self) -> Self { *self }
    }
}

mod copy_to
{
    //! Holds the [`CopyTo`] trait.

    /// A simple trait that with `copy_to`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lfr_stdx::CopyTo;
    /// #
    /// let mut i = 1;
    /// assert_eq!(2, 2.copy_to(&mut i));
    /// assert_eq!(i, 2);
    /// ```
    pub trait CopyTo: Copy
    {
        /// Copies self to other and returns self
        #[inline(always)]
        fn copy_to(self, other: &mut Self) -> Self
        {
            *other = self;
            self
        }
    }

    impl<T: Copy> CopyTo for T {}
}

mod with
{
    //! Kotlin-like `let` expressions.
    //! But due to rust's `let` keyword, it's not possible
    //! to use it so they are renamed to `with`

    /// Trait to implement `with` on all types.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lfr_stdx::With;
    /// assert_eq!(2, 1.with(|it| it + 1));
    /// ```
    pub trait With: Sized
    {
        /// Convert this object with a closure
        #[inline(always)]
        fn with<F, T>(self, f: F) -> T
            where F: FnOnce(Self) -> T
        {
            f(self)
        }
    }

    /// Trait to implement `with_ref` on all types.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lfr_stdx::WithRef;
    /// assert_eq!(String::from("aaa"),
    ///            "aaa".with_ref(|it| it.to_string()));
    /// ```
    pub trait WithRef
    {
        /// Convert a reference with a closure
        #[inline(always)]
        fn with_ref<F, T>(&self, f: F) -> T
            where F: FnOnce(&Self) -> T
        {
            f(self)
        }
    }

    /// Trait to implement `with_ref_mut` on all types.
    /// 
    /// # Examples
    ///
    /// ```
    /// # use lfr_stdx::WithRefMut;
    /// # #[derive(Debug)]
    /// #[derive(Eq, PartialEq)]
    /// struct P(i32);
    /// let mut p = P(0);
    /// assert_eq!(1,
    ///            p.with_ref_mut(|it| {
    ///                 it.0 = 1;
    ///                 it.0
    ///             }));
    /// assert_eq!(P(1), p);
    /// ```
    pub trait WithRefMut
    {
        /// Convert a mutable reference with a closure
        #[inline(always)]
        fn with_ref_mut<F, T>(&mut self, f: F) -> T
            where F: FnOnce(&mut Self) -> T
        {
            f(self)
        }
    }

    impl<T> With for T {}
    impl<T> WithRef for T {}
    impl<T> WithRefMut for T {}
}

mod take_if_unless
{
    /// Defines `take_if` and `take_unless`, which return
    /// an option with the value depending on the
    /// condition
    pub trait TakeIfUnless: Sized
    {
        /// Returns `Some(...)` if condition == true or None otherwise
        #[inline(always)]
        fn take_if<F>(self, condition: F) -> Option<Self>
            where F: FnOnce(&Self) -> bool
        {
            if condition(&self) { Some(self) } else { None }
        }
        /// Returns `None` if condition == true or Some(...) otherwise
        #[inline(always)]
        fn take_unless<F>(self, condition: F) -> Option<Self>
            where F: FnOnce(&Self) -> bool
        {
            if condition(&self) { None } else { Some(self) }
        }
    }

    impl<T> TakeIfUnless for T {}
}

mod take_if_unless_owned
{
    /// Similar to [`TakeIfUnless`][`super::TakeIfUnless`], but works with the
    /// owned types
    pub trait TakeIfUnlessOwned: ToOwned
    {
        /// Similar to
        /// [`TakeIfUnless::take_if`][`super::TakeIfUnless::take_if`], but calls
        /// to_owned() too.
        #[inline(always)]
        fn take_if_owned<F>(&self, condition: F) -> Option<Self::Owned>
            where F: FnOnce(&Self) -> bool
        {
            if condition(self) {
                Some(self.to_owned())
            }
            else {
                None
            }
        }
        /// Similar to
        /// [`TakeIfUnless::take_unless`][`super::TakeIfUnless::take_unless`],
        /// but calls to_owned() too.
        #[inline(always)]
        fn take_unless_owned<F>(&self, condition: F) -> Option<Self::Owned>
            where F: FnOnce(&Self) -> bool
        {
            if condition(self) {
                None
            }
            else {
                Some(self.to_owned())
            }
        }
    }
}

pub use also::*;
pub use and_then_do::*;
pub use clone_to::*;
pub use copied_val::*;
pub use copy_to::*;
pub use take_if_unless::*;
pub use take_if_unless_owned::*;
pub use with::*;

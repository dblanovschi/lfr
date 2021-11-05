mod also {
    //! Kotlin-like `also` expressions.

    /// Trait to implement `also` on all types
    pub trait Also: Sized {
        /// Kotlin-like `also` extension function
        #[inline(always)]
        fn also<F, T>(mut self, f: F) -> Self
        where
            F: FnOnce(&mut Self) -> T,
        {
            let _ = f(&mut self);
            self
        }
    }

    impl<T> Also for T {}
}

mod and_then_do {
    /// A trait for exposing [`AndThenDo::and_then_do`] on [`Option`] and
    /// [`Result`]
    pub trait AndThenDo {
        /// The type of the value
        type VT;
        /// Do an action on the `Ok`/`Some` value and don't ignore
        /// the returned value
        fn and_then_do<F, U>(self, f: F)
        where
            F: FnOnce(Self::VT) -> U;
    }

    impl<T, E> AndThenDo for Result<T, E> {
        type VT = T;

        #[inline(always)]
        fn and_then_do<F, U>(self, f: F)
        where
            F: FnOnce(T) -> U,
        {
            if let Ok(v) = self {
                let _ = f(v);
            }
        }
    }

    impl<T> AndThenDo for Option<T> {
        type VT = T;

        #[inline(always)]
        fn and_then_do<F, U>(self, f: F)
        where
            F: FnOnce(T) -> U,
        {
            if let Some(v) = self {
                let _ = f(v);
            }
        }
    }
}

mod clone_to {
    //! Holds the [`CloneTo`] trait

    /// Trait to define `clone_to*` on all types
    pub trait CloneTo: Clone {
        /// Clones other from self and returns self; reference variant.
        ///
        /// # Examples
        /// ```
        /// # use leafbuild_stdx::CloneTo;
        /// let mut a = 0;
        /// assert_eq!(&1, 1_i32.clone_to_ref(&mut a));
        /// assert_eq!(1, a);
        /// ```
        #[inline(always)]
        fn clone_to_ref(&self, other: &mut Self) -> &Self {
            other.clone_from(self);
            self
        }

        /// Clones other from self and returns self; mutable reference variant.
        ///
        /// # Examples
        /// ```
        /// # use leafbuild_stdx::CloneTo;
        /// let mut a = 0;
        /// assert_eq!(&mut 1, 1_i32.clone_to_ref_mut(&mut a));
        /// assert_eq!(1, a);
        /// ```
        #[inline]
        fn clone_to_ref_mut(&mut self, other: &mut Self) -> &mut Self {
            other.clone_from(self);
            self
        }

        /// Clones other from self and returns self; owned variant.
        ///
        /// # Examples
        /// ```
        /// # use leafbuild_stdx::CloneTo;
        /// let mut a = 0;
        /// assert_eq!(1, 1_i32.clone_to(&mut a));
        /// assert_eq!(1, a);
        /// ```
        #[inline(always)]
        fn clone_to(self, other: &mut Self) -> Self {
            other.clone_from(&self);
            self
        }
    }

    impl<T: Clone> CloneTo for T {}
}

mod copied_val {
    /// Copies the value and returns it, so no need to go back and use `*`.
    pub trait CopiedVal: Sized + Copy {
        #[inline(always)]
        fn copied_val(&self) -> Self {
            *self
        }
    }
}

mod copy_to {
    //! Holds the [`CopyTo`] trait.

    /// A simple trait that with `copy_to`.
    pub trait CopyTo: Copy {
        /// Copies self to other and returns self
        ///
        /// # Examples
        /// ```
        /// # use leafbuild_stdx::CopyTo;
        /// #
        /// let mut i = 1;
        /// assert_eq!(2, 2.copy_to(&mut i));
        /// assert_eq!(i, 2);
        /// ```
        #[inline(always)]
        fn copy_to(self, other: &mut Self) -> Self {
            *other = self;
            self
        }
    }

    impl<T: Copy> CopyTo for T {}
}

mod let_ {
    //! Kotlin-like `let` expressions.

    /// Trait to implement `let` on all types.
    pub trait Let: Sized {
        /// Convert this object with a closure
        ///
        /// # Examples
        ///
        /// ```
        /// # use leafbuild_stdx::Let;
        /// assert_eq!(2, 1.let_(|it| it + 1));
        /// ```
        #[inline(always)]
        fn let_<F, T>(self, f: F) -> T
        where
            F: FnOnce(Self) -> T,
        {
            f(self)
        }
    }

    /// Trait to implement `let_ref` on all types.
    pub trait LetRef {
        /// Convert a reference with a closure
        ///
        /// # Examples
        ///
        /// ```
        /// # use leafbuild_stdx::LetRef;
        /// assert_eq!(String::from("aaa"), "aaa".let_ref(|it| it.to_string()));
        /// ```
        #[inline(always)]
        fn let_ref<F, T>(&self, f: F) -> T
        where
            F: FnOnce(&Self) -> T,
        {
            f(self)
        }
    }

    /// Trait to implement `let_ref_mut` on all types.
    pub trait LetRefMut {
        /// Convert a mutable reference with a closure
        ///
        /// # Examples
        ///
        /// ```
        /// # use leafbuild_stdx::LetRefMut;
        /// # #[derive(Debug)]
        /// #[derive(Eq, PartialEq)]
        /// struct P(i32);
        /// let mut p = P(0);
        /// assert_eq!(
        ///     1,
        ///     p.let_ref_mut(|it| {
        ///         it.0 = 1;
        ///         it.0
        ///     })
        /// );
        /// assert_eq!(P(1), p);
        /// ```
        #[inline(always)]
        fn let_ref_mut<F, T>(&mut self, f: F) -> T
        where
            F: FnOnce(&mut Self) -> T,
        {
            f(self)
        }
    }

    impl<T> Let for T {}
    impl<T> LetRef for T {}
    impl<T> LetRefMut for T {}
}

mod take_if_unless {
    /// Defines `take_if` and `take_unless`, which return
    /// an option with the value depending on the
    /// condition
    pub trait TakeIfUnless: Sized {
        /// Returns `Some(...)` if condition == true or None otherwise
        #[inline(always)]
        fn take_if<F>(self, condition: F) -> Option<Self>
        where
            F: FnOnce(&Self) -> bool,
        {
            if condition(&self) { Some(self) } else { None }
        }
        /// Returns `None` if condition == true or Some(...) otherwise
        #[inline(always)]
        fn take_unless<F>(self, condition: F) -> Option<Self>
        where
            F: FnOnce(&Self) -> bool,
        {
            if condition(&self) { None } else { Some(self) }
        }
    }

    impl<T> TakeIfUnless for T {}
}

mod take_if_unless_owned {
    /// Similar to [`TakeIfUnless`][`super::TakeIfUnless`], but works with the
    /// owned types
    pub trait TakeIfUnlessOwned: ToOwned {
        /// Similar to
        /// [`TakeIfUnless::take_if`][`super::TakeIfUnless::take_if`], but calls
        /// to_owned() too.
        #[inline(always)]
        fn take_if_owned<F>(&self, condition: F) -> Option<Self::Owned>
        where
            F: FnOnce(&Self) -> bool,
        {
            if condition(self) {
                Some(self.to_owned())
            } else {
                None
            }
        }
        /// Similar to
        /// [`TakeIfUnless::take_unless`][`super::TakeIfUnless::take_unless`],
        /// but calls to_owned() too.
        #[inline(always)]
        fn take_unless_owned<F>(&self, condition: F) -> Option<Self::Owned>
        where
            F: FnOnce(&Self) -> bool,
        {
            if condition(self) {
                None
            } else {
                Some(self.to_owned())
            }
        }
    }
}

pub use self::also::*;
pub use self::and_then_do::*;
pub use self::clone_to::*;
pub use self::copied_val::*;
pub use self::copy_to::*;
pub use self::let_::*;
pub use self::take_if_unless::*;
pub use self::take_if_unless_owned::*;

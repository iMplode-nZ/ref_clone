//! This crate provides an implementation of a borrow as a higher kinded type.
//!
//! This can be used to abstract over the type of borrow by passing the type of the borrow as a type argument.

use std::marker::PhantomData;

/// The type of the borrow.
///
/// This may either be Immutable or Mutable.
pub trait RefType: private::Sealed + Copy {}

/// The Ref type. Third type parameter is the type of the Borrow.
#[derive(PartialEq, Eq)]
pub struct Ref<'a, T, S: RefType> {
    pub ty: S,
    pub value: &'a T,
    _no_pub_constructor: PhantomData<()>,
}

impl<'a, T, S: RefType> Ref<'a, T, S> {

    /// Converts the Ref into a borrow. This works for both mutable and immutable references.
    #[inline(always)]
    pub fn to_borrow(self) -> &'a T {
        self.value
    }

    #[inline(always)]
    pub unsafe fn new(ty: S, value: &'a T) -> Ref<'a, T, S> {
        Ref {
            ty,
            value,
            _no_pub_constructor: PhantomData,
        }
    }
}

impl<'a, T> Ref<'a, T, Mutable> {
    /// Converts the Ref into a mutable borrow. This only works for mutable references.
    #[inline(always)]
    pub fn to_mut_borrow(self) -> &'a mut T {
        unsafe { (self.value as *const T as *mut T).as_mut().unwrap() }
    }
}

/// Immutable Reference type.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Immutable;
// Mutable Reference type.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Mutable;

impl RefType for Immutable {}

impl RefType for Mutable {}

impl Immutable {
    /// Creates a new immutable Ref from a borrow.
    #[inline(always)]
    pub fn new<'a, T>(t: &'a T) -> Ref<'a, T, Immutable> {
        Ref {
            ty: Immutable,
            value: t,
            _no_pub_constructor: PhantomData,
        }
    }
}

impl Mutable {
    /// Creates a new mutable Ref from a mutable borrow.
    #[inline(always)]
    pub fn new<'a, T>(t: &'a mut T) -> Ref<'a, T, Mutable> {
        Ref {
            ty: Mutable,
            value: t,
            _no_pub_constructor: PhantomData,
        }
    }
}

impl<'a, T: std::fmt::Debug, S: RefType> std::fmt::Debug for Ref<'a, T, S> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.value.fmt(formatter)
    }
}

impl<'a, T: std::fmt::Display, S: RefType> std::fmt::Display for Ref<'a, T, S> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.value.fmt(formatter)
    }
}

mod private {
    use crate::*;

    pub trait Sealed {}

    impl Sealed for Immutable {}
    impl Sealed for Mutable {}
    impl<T, S: RefType> Sealed for Ref<'_, T, S> {}
}

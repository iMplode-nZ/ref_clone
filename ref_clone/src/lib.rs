//! This crate provides an implementation of a borrow as a higher kinded type.
//!
//! This can be used to abstract over the type of borrow by passing the type of the borrow as a type argument.
//!
//! Example:
//!
//! ```
//! #[RefAccessors]
//! struct Example {
//!     pub value: u8,
//! }
//! fn get_example_value<'a, T: RefType>(x: Ref<'a, Example, T>) -> Ref<'a, u8, T> {
//!     let x = x.to_wrapped();
//!     x.value
//! }
//! fn main() {
//!     let mut ex = Example {
//!         value: 8
//!     };
//!     {
//!         let ex_ref = Shared::new(&ex);
//!         println!("{}", get_example_value(ex_ref)); // = 8
//!     }
//!     {
//!         let ex_mut = Unique::new(&mut ex);
//!         *get_example_value(ex_mut).as_mut() = 1;
//!     }
//!     println!("{}", ex.value); // = 1
//!     {
//!         let ex_ref = Shared::new(&ex);
//!         println!("{}", get_example_value(ex_ref)); // = 1
//!     }
//! }
//! ```

#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(arbitrary_self_types)]
#![feature(generic_associated_types)]

use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use std::slice::Iter;

/// The type of the borrow.
///
/// This may either be Shared or Unique.
pub trait RefType: private::Sealed + Copy {}

/// The Ref type. Third type parameter is the type of the Borrow.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Ref<'a, T: ?Sized, S: RefType> {
    value: &'a T,
    ty: PhantomData<S>,
}

impl<'a, T: ?Sized, S: RefType> Deref for Ref<'a, T, S> {
    type Target = T;
    fn deref(&self) -> &T {
        self.as_ref()
    }
}

impl<'a, T: ?Sized> DerefMut for Ref<'a, T, Unique> {
    fn deref_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<'a, T: ?Sized, S: RefType> Ref<'a, T, S> {
    /// Converts the Ref into a borrow. This works for both shared and unique references.
    #[inline(always)]
    pub fn as_ref(&self) -> &'a T {
        self.value
    }

    /// UNSAFE. Do not use unless you know exactly what you are doing.
    /// Use of this to create a Unique reference (`Ref<'a, T, Unique>`) is undefined behaviour.
    ///
    /// This is only public so that ref_clone_derive can call it.
    #[inline(always)]
    pub unsafe fn __new_unsafe(value: &'a T) -> Ref<'a, T, S> {
        Ref {
            value,
            ty: PhantomData,
        }
    }

    /// UNSAFE. Do not use unless you know exactly what you are doing.
    #[inline(always)]
    pub unsafe fn __value(self) -> &'a T {
        self.value
    }
}

impl<'a, T: ?Sized> Ref<'a, T, Unique> {
    /// Converts the Ref into a mutable borrow. This only works for shared references.
    #[inline(always)]
    pub fn as_mut(&mut self) -> &'a mut T {
        unsafe { (self.value as *const T as *mut T).as_mut().unwrap() }
    }
}

/// Shared Reference type.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Shared;
// Unique Reference type.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Unique;

impl RefType for Shared {}

impl RefType for Unique {}

impl Shared {
    /// Creates a new shared Ref from a shared borrow.
    #[inline(always)]
    pub fn new<'a, T: ?Sized>(t: &'a T) -> Ref<'a, T, Shared> {
        Ref {
            value: t,
            ty: PhantomData,
        }
    }
}

impl Unique {
    /// Creates a new unique Ref from a unique borrow.
    #[inline(always)]
    pub fn new<'a, T: ?Sized>(t: &'a mut T) -> Ref<'a, T, Unique> {
        Ref {
            value: t,
            ty: PhantomData,
        }
    }
}

pub trait IntoRef {
    type Output;
    fn into_ref(self) -> Self::Output;
}

impl<'a, T: ?Sized> IntoRef for &'a T {
    type Output = Ref<'a, T, Shared>;
    fn into_ref(self) -> Self::Output {
        Shared::new(self)
    }
}

impl<'a, T: ?Sized> IntoRef for &'a mut T {
    type Output = Ref<'a, T, Unique>;
    fn into_ref(self) -> Self::Output {
        Unique::new(self)
    }
}

impl<'a, T: ?Sized, S: RefType> Ref<'a, T, S> {
    pub fn new(this: impl IntoRef<Output = Self>) -> Self {
        this.into_ref()
    }
}

pub trait RefAccessors<Wrapped> {
    fn to_wrapped(self) -> Wrapped;
}

impl<'a, T: std::fmt::Debug + ?Sized, S: RefType> std::fmt::Debug for Ref<'a, T, S> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.value.fmt(formatter)
    }
}

impl<'a, T: std::fmt::Display + ?Sized, S: RefType> std::fmt::Display for Ref<'a, T, S> {
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

    impl Sealed for Shared {}
    impl Sealed for Unique {}
    impl<T, S: RefType> Sealed for Ref<'_, T, S> {}
}

mod traits;
pub use traits::*;

/* =============== Specific implementation of traits =============== */

impl<T> IndexRef<usize> for [T] {
    // Array ref
    type Output = T;
    fn index_ref<'a, S: RefType>(self: Ref<'a, Self, S>, i: usize) -> Ref<'a, T, S> {
        unsafe { Ref::__new_unsafe(&self.value[i]) }
    }
}

impl<T, const N: usize> IndexRef<usize> for [T; N] {
    // Array ref
    type Output = T;
    fn index_ref<'a, S: RefType>(self: Ref<'a, Self, S>, i: usize) -> Ref<'a, T, S> {
        unsafe { Ref::__new_unsafe(&self.value[i]) }
    }
}

impl<T> DerefRef for Box<T> {
    type Target = T;
    fn deref_ref<'a, S: RefType>(self: Ref<'a, Self, S>) -> Ref<'a, T, S> {
        unsafe { Ref::__new_unsafe(self.__value().deref()) }
    }
}

pub struct RefIter<'a, T, S: RefType> {
    iter: Iter<'a, T>,
    _marker: PhantomData<S>,
}

impl<'a, T, S: RefType> Iterator for RefIter<'a, T, S> {
    type Item = Ref<'a, T, S>;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match self.iter.next() {
                Some(a) => Some(Ref::__new_unsafe(a)),
                None => None,
            }
        }
    }
}

impl<'a, T: 'a, S: RefType> IntoIteratorRef<'a, S> for [T] {
    type Item =  T;
    type IntoIter = RefIter<'a, T, S>;
    fn into_iter_ref(self: Ref<'a, Self, S>) -> Self::IntoIter {
        RefIter {
            iter: unsafe { self.__value().into_iter() },
            _marker: PhantomData,
        }
    }
}

impl<'a, T: 'a, S: RefType, const N: usize> IntoIteratorRef<'a, S> for [T; N] {
    type Item = T;
    type IntoIter = RefIter<'a, T, S>;
    fn into_iter_ref(self: Ref<'a, Self, S>) -> Self::IntoIter {
        RefIter {
            iter: unsafe { self.__value().into_iter() },
            _marker: PhantomData,
        }
    }
}

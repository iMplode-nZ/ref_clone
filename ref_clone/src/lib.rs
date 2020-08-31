use std::marker::PhantomData;

pub struct Immutable<'a, T>(pub &'a T);
pub struct Mutable<'a, T>(pub &'a mut T);
pub type Mut<'a, T> = Mutable<'a, T>;

pub struct Ref<'a, T, X = Immutable<'a, T>> {
    pub x: X,
    _marker: PhantomData<&'a T>
}

impl<'a, T> Ref<'a, T, Immutable<'a, T>> {
    pub fn to_borrow(self) -> &'a T {
        self.x.0
    }
}

impl<'a, T> Ref<'a, T, Mutable<'a, T>> {
    pub fn to_borrow(self) -> &'a T {
        self.x.0
    }

    pub fn to_borrow_mut(self) -> &'a mut T {
        self.x.0
    }
}

mod private {
    use crate::*;

    pub trait Sealed {}

    impl<T> Sealed for Immutable<'_, T> {}
    impl<T> Sealed for Mutable<'_, T> {}
}
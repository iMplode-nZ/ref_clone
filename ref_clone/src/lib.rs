pub trait HKT<Interior> {
    type From;
    type To;
}

pub trait Ref<'a, T, U> : HKT<U> + private::Sealed {
    fn to_borrow(self) -> &'a T;
    fn ty(self) -> bool;
}

pub trait RefMut<'a, T, U> : Ref<'a, T, U> {
    fn to_borrow_mut(self) -> &'a mut T;
}
pub struct Borrow<'a, T>(pub &'a T);

impl<'a, F, T: 'a> HKT<T> for Borrow<'a, F> {
    type From = Borrow<'a, F>;
    type To = Borrow<'a, T>;
}

impl<'a, T, U: 'a> Ref<'a, T, U> for Borrow<'a, T> {
    fn to_borrow(self) -> &'a T {
        self.0
    }
    fn ty(self) -> bool { false }
}

impl<'a, T> Borrow<'a, T> {
    pub fn new(x: &'a T) -> Self {
        Borrow(x)
    }
}

pub struct BorrowMut<'a, T>(pub &'a mut T);

impl<'a, F, T: 'a> HKT<T> for BorrowMut<'a, F> {
    type From = BorrowMut<'a, F>;
    type To = BorrowMut<'a, T>;
}

impl<'a, T, U: 'a> Ref<'a, T, U> for BorrowMut<'a, T> {
    fn to_borrow(self) -> &'a T {
        self.0
    }

    fn ty(self) -> bool { true }
}

impl<'a, T, U: 'a> RefMut<'a, T, U> for BorrowMut<'a, T> {
    fn to_borrow_mut(self) -> &'a mut T {
        self.0
    }
}

impl<'a, T> BorrowMut<'a, T> {
    pub fn new(x: &'a mut T) -> Self {
        BorrowMut(x)
    }
}

mod private {
    use crate::*;

    pub trait Sealed {}

    impl<T> Sealed for Borrow<'_, T> {}
    impl<T> Sealed for BorrowMut<'_, T> {}
}
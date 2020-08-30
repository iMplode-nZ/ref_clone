pub trait HKT<Interior> {
    type From;
    type To;
}

pub trait Ref<'a, T, U> : HKT<U> {
    fn to_borrow(self) -> &'a T;
}

pub trait RefMut<'a, T> {
    fn to_borrow_mut(self) -> &'a mut T;
}
pub struct Borrow<'a, T> {
    x: &'a T,
}

impl<'a, F, T: 'a> HKT<T> for Borrow<'a, F> {
    type From = Borrow<'a, F>;
    type To = Borrow<'a, T>;
}

impl<'a, T, U: 'a> Ref<'a, T, U> for Borrow<'a, T> {
    fn to_borrow(self) -> &'a T {
        self.x
    }
}

impl<'a, T> Borrow<'a, T> {
    pub fn new(x: &'a T) -> Self {
        Borrow { x }
    }
}

pub struct BorrowMut<'a, T> {
    x: &'a mut T,
}

impl<'a, F, T: 'a> HKT<T> for BorrowMut<'a, F> {
    type From = BorrowMut<'a, F>;
    type To = BorrowMut<'a, T>;
}

impl<'a, T, U: 'a> Ref<'a, T, U> for BorrowMut<'a, T> {
    fn to_borrow(self) -> &'a T {
        self.x
    }
}

impl<'a, T> RefMut<'a, T> for BorrowMut<'a, T> {
    fn to_borrow_mut(self) -> &'a mut T {
        self.x
    }
}

impl<'a, T> BorrowMut<'a, T> {
    pub fn new(x: &'a mut T) -> Self {
        BorrowMut { x }
    }
}

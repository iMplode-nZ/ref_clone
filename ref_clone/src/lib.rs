pub trait Ref<'a, T> {
    fn to_borrow(self) -> &'a T;
}

pub trait RefMut<'a, T> {
    fn to_borrow_mut(self) -> &'a mut T;
}
pub struct Borrow<'a, T> {
    x: &'a T,
}

impl<'a, T, LiftTarget1: 'a> ::higher::Lift<T, LiftTarget1> for Borrow<'a, T> {
    type Source = Self;
    type Target1 = Borrow<'a, LiftTarget1>;
}
impl<'a, T, LiftTarget1: 'a, LiftTarget2: 'a> ::higher::Lift3<T, LiftTarget2, LiftTarget1>
    for Borrow<'a, T>
{
    type Target2 = Borrow<'a, LiftTarget2>;
}

impl<'a, T> Ref<'a, T> for Borrow<'a, T> {
    fn to_borrow(self) -> &'a T {
        self.x
    }
}

impl<'a, T> Borrow<'a, T> {
    pub fn from_borrow(x: &'a T) -> Self {
        Borrow { x }
    }
    pub fn from_borrow_mut(x: &'a mut T) -> Self {
        Borrow { x }
    }
}
impl<'a, T, LiftTarget1: 'a> ::higher::Lift<T, LiftTarget1> for BorrowMut<'a, T> {
    type Source = Self;
    type Target1 = BorrowMut<'a, LiftTarget1>;
}
impl<'a, T, LiftTarget1: 'a, LiftTarget2: 'a> ::higher::Lift3<T, LiftTarget2, LiftTarget1>
    for BorrowMut<'a, T>
{
    type Target2 = BorrowMut<'a, LiftTarget2>;
}

pub struct BorrowMut<'a, T> {
    x: &'a mut T,
}

impl<'a, T> Ref<'a, T> for BorrowMut<'a, T> {
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
    pub fn from_borrow_mut(x: &'a mut T) -> Self {
        BorrowMut { x }
    }
}

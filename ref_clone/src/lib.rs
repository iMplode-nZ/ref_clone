pub trait RefType: private::Sealed {}
#[derive(PartialEq, Eq)]
pub struct Ref<'a, T, S: RefType> {
    pub ty: S,
    pub value: &'a T,
}

impl<'a, T, S: RefType> Ref<'a, T, S> {
    #[inline(always)]
    pub fn to_borrow(self) -> &'a T {
        self.value
    }
}

impl<'a, T> Ref<'a, T, Mutable> {
    pub fn to_mut_borrow(self) -> &'a mut T {
        unsafe { (self.value as *const T as *mut T).as_mut().unwrap() }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Immutable;
#[derive(Debug, PartialEq, Eq)]
pub struct Mutable;

impl RefType for Immutable {}

impl RefType for Mutable {}

impl Immutable {
    pub fn new<'a, T>(t: &'a T) -> Ref<'a, T, Immutable> {
        Ref {
            ty: Immutable,
            value: t,
        }
    }
}

impl Mutable {
    pub fn new<'a, T>(t: &'a mut T) -> Ref<'a, T, Mutable> {
        Ref {
            ty: Mutable,
            value: t,
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

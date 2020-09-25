use crate::*;

pub trait DerefRef {
    type Target: ?Sized;

    fn deref_ref<'a, S: RefType>(self: Ref<'a, Self, S>) -> Ref<'a, Self::Target, S>;
}

pub trait IndexRef<Idx> {
    type Output: ?Sized;

    fn index_ref<'a, S: RefType>(self: Ref<'a, Self, S>, index: Idx) -> Ref<'a, Self::Output, S>;
}

pub trait IntoIteratorRef<'a, T: RefType> {
    type Item: 'a;
    type IntoIter: Iterator<Item = Ref<'a, Self::Item, T>>;
    fn into_iter(self: Ref<'a, Self, T>) -> Self::IntoIter;
}

impl<'a, S, T: RefType> IntoIterator for Ref<'a, S, T> where S: IntoIteratorRef<'a, T> {
    type Item = Ref<'a, S::Item, T>;
    type IntoIter = S::IntoIter;
    fn into_iter(self) -> S::IntoIter {
        self.into_iter()
    }
}

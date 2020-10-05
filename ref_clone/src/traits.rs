use crate::*;

pub trait DerefRef {
    type Target: ?Sized;

    fn deref_ref<'a, S: RefType>(self: Ref<'a, Self, S>) -> Ref<'a, Self::Target, S>;
}

pub trait IndexRef<Idx> {
    type Output: ?Sized;

    fn index_ref<'a, S: RefType>(self: Ref<'a, Self, S>, index: Idx) -> Ref<'a, Self::Output, S>;
}

#[repr(transparent)]
pub struct RefIterator<'a, T: RefType, S, I: 'a>(S) where S: Iterator<Item = Ref<'a, I, T>>;

impl<'a, S: Iterator<Item = Ref<'a, I, Shared>>, I> Iterator for RefIterator<'a, Shared, S, I> {
    type Item = &'a I;

    fn next(&mut self) -> Option<&'a I> {
        self.0.next().map(|x| x.as_ref())
    }
}

impl<'a, S: Iterator<Item = Ref<'a, I, Unique>>, I> Iterator for RefIterator<'a, Unique, S, I> {
    type Item = &'a mut I;

    fn next(&mut self) -> Option<&'a mut I> {
        self.0.next().map(|mut x| x.as_mut())
    }
}

pub trait IntoIteratorRef<'a> {
    type Item: 'a;
    type IntoIter<T: RefType>: Iterator<Item = Ref<'a, Self::Item, T>>;
    fn into_iter_ref<T: RefType>(self: Ref<'a, Self, T>) -> Self::IntoIter<T>;
    fn iter(&'a self) -> RefIterator<'a, Shared, Self::IntoIter<Shared>, Self::Item> {
        RefIterator(Ref::new(self).into_iter_ref())
    }
    fn iter_mut(&'a mut self) -> RefIterator<'a, Unique, Self::IntoIter<Unique>, Self::Item> {
        RefIterator(Ref::new(self).into_iter_ref())
    }
}

impl<'a, S, T: RefType> IntoIterator for Ref<'a, S, T> where S: IntoIteratorRef<'a> {
    type Item = Ref<'a, S::Item, T>;
    type IntoIter = S::IntoIter<T>;
    fn into_iter(self) -> S::IntoIter<T> {
        self.into_iter_ref()
    }
}

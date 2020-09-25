use crate::*;


pub trait DerefRef {
    type Target;

    fn deref_ref<'a, S: RefType>(self: Ref<'a, Self, S>) -> Ref<'a, Self::Target, S> where Self: Sized;
}
/*
impl<T: DerefRef> Deref for T {
    fn deref(&self) -> &Self::Target {

    }
}
*/
pub trait IndexRef<Idx> {
    type Output;

    fn index_ref<'a, S: RefType>(self: Ref<'a, Self, S>, index: Idx) -> Ref<'a, Self::Output, S> where Self: Sized;
}
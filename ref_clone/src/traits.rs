use crate::*;

pub trait DerefRef {
    type Target: ?Sized;

    fn deref_ref<'a, S: RefType>(self: Ref<'a, Self, S>) -> Ref<'a, Self::Target, S>;
}

pub trait IndexRef<Idx> {
    type Output: ?Sized;

    fn index_ref<'a, S: RefType>(self: Ref<'a, Self, S>, index: Idx) -> Ref<'a, Self::Output, S>;
}

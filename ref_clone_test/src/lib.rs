/*#[macro_use]
extern crate ref_clone_derive;

#[cfg(test)]
mod tests {
    use ref_clone::*;
    pub trait RefFoox<'a, T> {
        fn get_x(self) -> Ref<'a, i64, T>;
    }
    impl<'a, T> RefFoox<'a, T> for Ref<'a, Foo, T> {
        fn get_x(self) -> Ref<'a, i64, T> {
            unsafe {
                /* TODO: THIS DOES NOT WORK DUE TO THE TYPE PARAMETER INCLUDING THE TYPES OF THE OTHER STUFF. MAKE IMMUTABLE AND MUTABLE struct Immutable(); INSTEAD OF HAVING CONTENTS */
                *(&&(*(&self.x as *const _ as *const &Foo)).x as *const _ as *const Ref<'a, i64, T>)
            }
        }
    }
    trait RefFooy<'a, T> {
        fn get_y(self) -> Ref<'a, Vec<u32>, T>;
    }
    impl<'a, T> RefFooy<'a, T> for Ref<'a, Foo, T> {
        fn get_y(self) -> Ref<'a, Vec<u32>, T> {
            unsafe {
                *(&&((*(&self.x as *const _ as *const &Foo)).y) as *const _
                    as *const Ref<'a, Vec<u32>, T>)
            }
        }
    }

    //#[derive(RefAccessors)]
    struct Foo {
        pub x: i64,
        y: Vec<u32>,
    }
}
*/
pub trait FunctorShape {
    unsafe fn map<A, B, F: Fn(A) -> B>(&self, a: *const (), f: F) -> *const ();
}
pub struct Functor<T, S: FunctorShape> {
    x: S,
    value: *const (),
    _marker: std::marker::PhantomData<T>
}

impl<A, S: FunctorShape> Functor<A, S> {
    pub fn map<B, F: Fn(A) -> B>(self, f: F) -> Functor<B, S> {
        let value = unsafe { self.x.map(self.value, f) };
        Functor {
            x: self.x,
            value,
            _marker: std::marker::PhantomData
        }
    }

    pub unsafe fn new<X>(a: X, x: S) -> Functor<A, S> {
        let value = std::boxed::Box::<X>::into_raw(Box::new(a)) as *const ();
        Functor {
            x,
            value,
            _marker: std::marker::PhantomData
        }
    }
}

pub struct VecFunctor;

impl FunctorShape for VecFunctor {
    unsafe fn map<A, B, F: Fn(A) -> B>(&self, a: *const (), f: F) -> *const () {
        std::mem::transmute(&std::mem::transmute_copy::<_, Vec<A>>(a.as_ref().unwrap()).into_iter().map(f).collect::<Vec<B>>())
    }
}

impl VecFunctor {
    pub fn to_functor<T>(a: Vec<T>) -> Functor<T, VecFunctor> {
        unsafe {
            Functor::new(a, VecFunctor)
        }
    }
}

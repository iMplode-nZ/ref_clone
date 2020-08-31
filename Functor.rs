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

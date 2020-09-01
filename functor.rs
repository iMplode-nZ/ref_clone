pub trait FunctorShape {
    unsafe fn map<A, B, F: Fn(A) -> B>(&self, a: *mut (), f: F) -> *mut ();
}
pub struct Functor<T, S: FunctorShape> {
    x: S,
    value: *mut (),
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
        let value = Box::into_raw(Box::new(a)) as *mut ();
        Functor {
            x,
            value,
            _marker: std::marker::PhantomData
        }
    }
}

pub struct VecFunctor;

impl FunctorShape for VecFunctor {
    unsafe fn map<A, B, F: Fn(A) -> B>(&self, a: *mut (), f: F) -> *mut () {
        Box::into_raw(Box::new(Box::from_raw(a as *mut Vec<A>).into_iter().map(f).collect::<Vec<B>>())) as *mut ()
    }
}

impl VecFunctor {
    pub fn to_functor<T>(a: Vec<T>) -> Functor<T, VecFunctor> {
        unsafe {
            Functor::new(a, VecFunctor)
        }
    }

    pub fn from_functor<T>(a: Functor<T, VecFunctor>) -> Vec<T> {
        unsafe {
            *Box::from_raw(a.value as *mut Vec<T>)
        }
    }
}

#[test]
fn test_functor() {
    let func = VecFunctor::to_functor(vec![1, 2, 3, 4]);
    let b = func.map(|x| x.to_string());
    println!("{:?}", VecFunctor::from_functor(b));
}
<!-- markdownlint-disable no-inline-html no-bare-urls line-length header-increment -->

# An Alternative Higher Kinded Type Implementation in Rust

## The Standard Way

The standard way of implementing higher kinded types is by using a `Lift` trait. This trait has the following form:

```rust
trait Lift<From, To> {
    type Source;
    type Target;
}
```

The lift type can be implemented on a type as such:

```rust
impl<F, T> Lift<F, T> for Option<F> {
    type Source = Self;
    type Target = Option<T>;
}
```

Then, this can be used to make a Functor:

```rust
pub trait Functor<A, B>: Lift<A, B> {
    fn map<F>(self, f: F) -> <Self as Lift<A, B>>::Target
    where
        F: Fn(A) -> B;
}
```

As long as the thing is known to be a `Lift<A, B>` and a `Functor<A, B>`, then it can be mapped over.

### Issues

There are a couple of issues with this. One particularly big issue is that there is no guarantee that the output from calling `map` on a functor is still a functor. The `Target` type has no bounds, and as such it could be anything. For example, someone may implement a `Lift` as such:

```rust
impl<F, T> Lift<F, T> for Option<F> {
    type Source = Self;
    type Target = ();
}
```

There is no guarantee that the `::Target` is actually of type `Lift` and of type `Functor`. <!-- TODO: TALK ABOUT SECURING THIS USING WHERE -->

## Alternative

This alternative uses a wrapper type to move the type argument to the type into a seperate argument so that they can be abstracted over seperately. For example, take the case of a `Functor`. With this method, the `Functor` would have this form:

```rust
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
```

This then would be implemented for a `Vec` as such:

```rust
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

```

While this code is extremely ugly, as long as `a` starts out as a valid pointer to a `Vec<A>`, it ends up as a valid pointer to a `Vec<B>`. As such, as long as the creation is valid, the entirety of all the function calls on this `Functor` are valid. I have also provided an example creation method.

One advantage of this approach is that as the shape of the type is passed seperately, it can be changed independently of the type of the interior. This approach has a couple of disadvantages though, namely that it is extremely unsafe and a little less performant.

For example, one use of it could be a function that applies a map to a functor. In contrast to the other implementation, this one may be chained arbitrarily:

```rust
fn box_all<A, S: FunctorShape>(x: Functor<A, S>) -> Functor<Box<A>, S> {
    x.map(|a| Box::new(a))
}
```

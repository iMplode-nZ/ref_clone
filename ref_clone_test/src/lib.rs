#[macro_use]
extern crate ref_clone_derive;

#[cfg(test)]
mod tests {
    use ref_clone::Ref;
    // #[derive(RefAccessors)]
    struct Foo {
        pub x: i64,
        y: Vec<u32>,
    }
    impl Foo {
        pub fn get_x<'a, T: ::ref_clone::Ref<'a, Self, i64>>(
            this: T,
        ) -> <T as ::ref_clone::HKT<i64>>::To {
            match this.ty() {
                false => ::ref_clone::Borrow(&this.to_borrow().x),
                true => ::ref_clone::BorrowMut(unsafe {
                    (&this.to_borrow().x as *const i64 as *mut i64)
                        .as_ref()
                        .unwrap()
                }),
            }
        }
        fn get_y<'a, T: ::ref_clone::Ref<'a, Self, Vec<u32>>>(
            this: T,
        ) -> <T as ::ref_clone::HKT<Vec<u32>>>::To {
            match this.ty() {
                false => ::ref_clone::Borrow(&this.to_borrow().y),
                true => ::ref_clone::BorrowMut(unsafe {
                    (&this.to_borrow().y as *const Vec<u32> as *mut Vec<u32>)
                        .as_ref()
                        .unwrap()
                }),
            }
        }
    }

    /*fn do_thing<'a, T: Ref<'a, Foo, i64>, U>(x: T) -> impl Ref<'a, i64, U>
    where
        <T as ref_clone::HKT<i64>>::To: ref_clone::Ref<'a, i64, U>,
    {
        Foo::get_x(x)
    }

    #[test]
    fn prob_throws() {
        let mut x = Foo { x: 10, y: vec![30] };
        let x = Foo::get_y(ref_clone::Borrow::new(&mut x));
    }*/
}

#[macro_use]
extern crate ref_clone_derive;

#[cfg(test)]
mod tests {
    use ref_clone::Ref;
    #[derive(RefAccessors)]

    struct Foo {
        pub x: i64,
        y: Vec<u32>,
    }

    fn do_thing<'a, T: Ref<'a, Foo, i64>, U>(x: T) -> impl Ref<'a, i64, U> {
        Foo::get_x(x)
    }

    #[test]
    fn prob_throws() {
        let mut x = Foo {
            x: 10,
            y: vec![30]
        };
        let x = Foo::get_y(ref_clone::Borrow::new(&mut x));
    }
}

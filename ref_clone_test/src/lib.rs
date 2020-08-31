#[macro_use]
extern crate ref_clone_derive;

#[cfg(test)]
mod tests {
    use ref_clone::*;

    #[derive(RefAccessors)]
    struct Foo {
        pub x: i64,
        y: Vec<u32>,
    }

    fn get_foo_child<S: RefType>(a: Ref<'_, Foo, S>) -> Ref<'_, i64, S> {
        a.get_x()
    }

    #[test]
    fn test() {
        let foo = Foo { x: 10, y: vec![3] };
        let r = Immutable::new(&foo);
        println!("{:?}", get_foo_child(r));
    }
}

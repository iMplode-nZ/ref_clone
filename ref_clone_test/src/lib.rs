#[cfg(test)]
mod tests {
    use ref_clone::*;
    use ref_clone_derive::RefAccessors;

    #[RefAccessors]
    struct Foo {
        pub x: i64,
        y: Vec<u32>,
    }

    fn get_foo_child<S: RefType>(a: Ref<'_, Foo, S>) -> Ref<'_, i64, S> {
        a.to_wrapped().x
    }

    fn get_foo_vec_child<S: RefType>(a: Ref<'_, Foo, S>) -> Ref<'_, Vec<u32>, S> {
        a.to_wrapped().y
    }

    #[test]
    fn test() {
        let foo = Foo { x: 10, y: vec![3] };
        let r = Immutable::new(&foo);
        assert_eq!(*get_foo_child(r).as_ref(), 10);
    }

    #[test]
    fn test_mut() {
        let mut foo = Foo { x: 13, y: vec![3] };
        let r = Mutable::new(&mut foo);
        assert_eq!(*get_foo_child(r).as_ref(), 13);
    }

    #[test]
    fn test_two() {
        let foo = Foo { x: 13, y: vec![3] };
        let r = Immutable::new(&foo);
        let r2 = Immutable::new(&foo);
        let r3 = Immutable::new(&foo);
        assert_eq!(*get_foo_child(r).as_ref(), 13);
        assert_eq!(*get_foo_child(r2).as_ref(), 13);
        assert_eq!(*get_foo_child(r3).as_ref(), 13);
    }

    #[test]
    fn test_vec() {
        let mut foo = Foo { x: 13, y: vec![3] };
        assert_eq!(
            get_foo_vec_child(Mutable::new(&mut foo)).as_mut()[0],
            3
        );
    }

    #[RefAccessors]
    struct FooGen<T> {
        pub a: T,
    }
    #[allow(dead_code)]
    #[RefAccessors]
    enum Enum<T> {
        Variant { x: u8 },
        OtherVariant { y: T },
    }

    #[RefAccessors]
    struct Example {
        pub value: u8,
    }

    fn get_example_value<'a, T: RefType>(x: Ref<'a, Example, T>) -> Ref<'a, u8, T> {
        let x = x.to_wrapped();
        x.value
    }

    #[test]
    fn main() {
        let mut ex = Example {
            value: 8
        };
        {
            let ex_ref = Immutable::new(&ex);
            println!("{}", get_example_value(ex_ref)); // = 8
        }
        {
            let ex_mut = Mutable::new(&mut ex);
            *get_example_value(ex_mut).as_mut() = 1;
        }
        println!("{}", ex.value); // = 1
        {
            let ex_ref = Immutable::new(&ex);
            println!("{}", get_example_value(ex_ref)); // = 1
        }
    }
}

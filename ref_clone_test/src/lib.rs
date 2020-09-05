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
        a.to_ref().x
    }

    fn get_foo_vec_child<S: RefType>(a: Ref<'_, Foo, S>) -> Ref<'_, Vec<u32>, S> {
        a.to_ref().y
    }

    #[test]
    fn test() {
        let foo = Foo { x: 10, y: vec![3] };
        let r = Immutable::new(&foo);
        assert_eq!(*get_foo_child(r).to_borrow(), 10);
    }

    #[test]
    fn test_mut() {
        let mut foo = Foo { x: 13, y: vec![3] };
        let r = Mutable::new(&mut foo);
        assert_eq!(*get_foo_child(r).to_borrow(), 13);
    }

    #[test]
    fn test_two() {
        let foo = Foo { x: 13, y: vec![3] };
        let r = Immutable::new(&foo);
        let r2 = Immutable::new(&foo);
        let r3 = Immutable::new(&foo);
        assert_eq!(*get_foo_child(r).to_borrow(), 13);
        assert_eq!(*get_foo_child(r2).to_borrow(), 13);
        assert_eq!(*get_foo_child(r3).to_borrow(), 13);
    }

    #[test]
    fn test_vec() {
        let mut foo = Foo { x: 13, y: vec![3] };
        assert_eq!(
            get_foo_vec_child(Mutable::new(&mut foo)).to_mut_borrow()[0],
            3
        );
    }

    #[RefAccessors]
    struct FooGen<T> {
        pub a: T,
    }

    enum Enum<T> {
        Variant { x: u8 },
        OtherVariant { y: T },
    }

    enum RefEnum<'a, T, A: ::ref_clone::RefType> {
        Variant { x: Ref<'a, u8, A> },
        OtherVariant { y: Ref<'a, T, A> },
    }

    impl<'a, A: ::ref_clone::RefType, T> RefAccessors<RefEnum<'a, T, A>> for Ref<'a, Enum<T>, A> {
        fn to_ref(self) -> RefEnum<'a, T, A> {
            match self.value {
                Enum::Variant { x } => RefEnum::Variant {
                    x: unsafe { Ref::new(x) },
                },
                Enum::OtherVariant { y } => RefEnum::OtherVariant {
                    y: unsafe { Ref::new(y) },
                },
            }
        }
    }
}

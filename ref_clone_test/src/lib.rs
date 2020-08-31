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

#[macro_use]
extern crate ref_clone_derive;
use ref_clone::*;

#[cfg(test)]
mod tests {
    #[derive(RefAccessors)]
    struct Foo {
        pub x: i64,
        y: Vec<u32>,
    }
}

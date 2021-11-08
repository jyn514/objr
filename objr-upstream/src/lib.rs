extern crate self as objr;

pub mod m {
    pub trait Trait {
        /// [objr]
        fn f() {}
    }
    impl Trait for usize {
        fn f() {}
    }
}
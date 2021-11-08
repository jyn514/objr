extern crate self as objr;

mod m {
    pub trait Trait {
        /// [objr]
        fn f() {}
    }
    impl Trait for usize {
        fn f() {}
    }
}

pub use m::Trait;
extern crate self as objr;

mod m {
    pub trait Trait {
        /// [objr]
        fn f() {}
    }
    impl <T> Trait for T {
        fn f() {}
    }
}

pub use m::Trait;
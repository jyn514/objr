pub mod m {
    pub trait Trait {
        /// [empty]
        fn f() {}
    }
    impl Trait for usize {
        fn f() {}
    }
}
//! # ClojuRust
//!
//! Proof of concept for a Clojure compiler in Rust.
//!
//! ##
#![allow(dead_code)]
#![allow(unused_variables)]
// ![warn(unreachable_pub, missing_docs)]
#![allow(missing_docs)]
#![allow(unused_imports)]
#![allow(bare_trait_objects)]
#![recursion_limit = "256"]
// #![feature(fn_traits)]

/// Clojure Module
pub mod clojure {
    /// Lang Module
    pub mod lang {
        // pub mod number;
    }
    pub mod rust {
        pub mod class;
        pub mod function;
        pub mod globals;
        pub mod object;
        pub mod phashmap;
        pub mod pvector;
        pub mod rustobj;
        pub mod str;
        pub mod unique;

        // pub mod implementation;
        // pub mod member;
        // pub mod protocol;
        // pub mod prototype;
        // pub mod hashset;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

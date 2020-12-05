//! # ClojuRust
//! 
//! Proof of concept for a Clojure compiler in Rust.
//!
//! ## 

#![recursion_limit="256"]
#![feature(fn_traits)]
#![allow(dead_code)]
#![allow(unused_variables)]
// ![warn(unreachable_pub, missing_docs)]
#![warn(missing_docs)]

#[cfg(test)]

pub(crate) mod rust {
    pub mod keywords;
    pub mod class;
    pub mod implementation;
    pub mod member;
    pub mod protocol;
    pub mod prototype;
    pub mod object;
    pub mod rust_obj;
}
pub(crate) mod clojure {
    pub  mod lang {
        // pub mod number;
        pub mod hashmap;
        pub mod hashset;
        pub mod vector;
        pub mod function;
    }
}

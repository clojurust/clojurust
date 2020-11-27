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

pub mod clojure {
    pub mod rust {
        pub mod keywords;
        pub mod class;
        pub mod function;
        pub mod implementation;
        pub mod member;
        pub mod protocol;
        pub mod prototype;
        pub mod object;
        pub mod rust_obj;
        pub mod hashmap;
        pub mod hashset;
        pub mod vector;
    }
    
    pub  mod lang {
        pub mod number;
    }
}

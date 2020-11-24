//! # ClojuRust
//! 
//! Proof of concept for a Clojure compiler in Rust.
//!
//! ## 

#![recursion_limit="256"]
#![feature(fn_traits)]
// #![warn(unreachable_pub, missing_docs)]
#![warn(missing_docs)]

#[cfg(test)]

pub mod clojure {
    pub mod rust {
        pub mod keywords;
        pub mod class;
        pub mod function;
        pub mod implementation;
        pub mod object;
        pub mod primitive;
        pub mod rust_obj;
    }
    
    pub  mod lang {
        pub mod number;
    }
}

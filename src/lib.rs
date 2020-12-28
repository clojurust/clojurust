//! # ClojuRust
//! 
//! Proof of concept for a Clojure compiler in Rust.
//!
//! ## 

#![recursion_limit="256"]
// #![feature(fn_traits)]
#![allow(dead_code)]
#![allow(unused_variables)]
// ![warn(unreachable_pub, missing_docs)]
#![warn(missing_docs)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Clojure Module
pub mod clojure {
/// Lang Module
    pub mod lang {
        // pub mod number;
    }
}
 

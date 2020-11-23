//! ClojuRust main crate

#![recursion_limit="256"]
#![feature(fn_traits)]
#[cfg(test)]

/// clojure crate
pub(crate) mod clojure {
    /// clojure module
    pub mod rust {
        /// keyword module
        pub mod keywords;
        pub mod class;
        pub mod function;
        pub mod implementation;
        pub mod object;
        pub mod primitive;
    }
    
    /// lang crate
    pub  mod lang {
        pub mod number;
    }
}

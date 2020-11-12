// extern crate proc_macro;
extern crate lazy_static;
extern crate mut_static;

#[cfg(test)]
// extern crate pretty_assertions;

// #[allow(unused_imports)]
// use proc_macro::TokenStream;

pub mod clojure {
    pub mod rust {
        pub mod keyword;
        pub mod object;
    }
    
    pub  mod lang {
        pub mod number;
    }
}

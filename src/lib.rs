// extern crate proc_macro;

#[cfg(test)]
// extern crate pretty_assertions;

// #[allow(unused_imports)]
// use proc_macro::TokenStream;

pub(crate) mod clojure {
    pub mod rust {
        pub mod keywords;
        pub mod object;
        pub mod primitive;
    }
    
    pub  mod lang {
        pub mod number;
    }
}

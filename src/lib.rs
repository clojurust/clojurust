extern crate proc_macro;

#[allow(unused_imports)]
use proc_macro::TokenStream;

pub mod rust {
    pub mod keyword;
}

pub mod clojure {
    pub  mod lang {
        pub mod object;
        pub mod number;
    }
}

:D
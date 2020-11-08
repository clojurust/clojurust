extern crate proc_macro;

#[allow(unused_imports)]
use proc_macro::TokenStream;

pub mod clojure {
    pub mod rust {
        pub mod keyword;
        pub mod object;
    }
    
    pub  mod lang {
        pub mod number;
    }
}

extern crate proc_macro;

#[allow(unused_imports)]
use proc_macro::TokenStream;

#[macro_use]
pub(crate) extern crate query_interface;

#[feature(in_band_lifetimes)]

pub(crate) 
mod clojure{
    pub(crate)  mod lang{
        pub(crate) mod object;
        pub(crate) mod number;
    }
}


//! # Protocol
use crate::clojure::lang::phashmap;

struct SProtocol {
    prototype: phashmap::SPHashMap,
    nb_arity: usize,
    multi_arity: bool,
}

pub fn init() {}

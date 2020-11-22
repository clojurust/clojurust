//! Anonymous Function with multi-arity

use im::vector::Vector;
use im::hashmap::HashMap;
// use std::sync::*;
// use lazy_static::lazy_static;
use super::implementation::*;
pub struct Function {
    min_arity: usize,
    max_arity: usize,
    func: HashMap<usize, Vector<Implementation>>,
}


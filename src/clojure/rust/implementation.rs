//! Anonymous Function with one defined arity, and possible last element multi arity

use im::vector::Vector;
// use im::hashmap::HashMap;
use std::sync::*;
// use lazy_static::lazy_static;
use super::object::*;

pub struct Implementation {
    arity: usize,
    multiary: bool,
    function: Vector<fn(&[Arc<Object>])>,
}
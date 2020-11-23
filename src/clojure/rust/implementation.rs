//! Anonymous Function with one defined arity, and possible last element multi arity

// use im::vector::Vector;
// use im::hashmap::HashMap;
use std::sync::*;
// use lazy_static::lazy_static;
use super::object::*;

#[derive(Clone)]
pub struct Implementation
{
    pub multiary: bool,
    pub function: &'static dyn Fn(&[Arc<Object>]) -> Arc<Object>,
}

impl Implementation 
{
    #[allow(dead_code)]
    pub fn new(multiary: bool, function: &'static dyn Fn(&[Arc<Object>]) -> Arc<Object>) -> Implementation {
        Implementation {
            multiary,
            function,
        }
    }

    #[allow(dead_code)]
    pub fn call(&self, args: &[Arc<Object>]) -> Arc<Object> {
        self.function.call((args,))
    }
}

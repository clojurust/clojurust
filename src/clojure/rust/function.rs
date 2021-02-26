//! Anonymous Function with multi-arity
use lazy_static::{__Deref, lazy_static};
use std::clone::Clone;
use std::{any::*, fmt::*, hash::*, result::*, sync::*};

// use std::fmt::*;
use intertrait::cast::*;
use intertrait::*;

use super::object::*;
use super::phashmap::*;
use super::pvector::*;
use super::rustobj::*;

pub struct Function {
    pub multiary: Option<usize>,
    pub func: PHashMap, // all implementations
}

castable_to!(Function => [sync] Object, IFunction);

trait IFunction {
    fn call(args: PVector) -> SObject {}

    fn get(&self, arity: usize) -> SObject {}
}

impl IFunction for Function {
    fn get(&self, arity: usize) -> SObject {
        match self.multiary {
            Some(max) => {
                if arity > max {
                    let implem = self.func.get(&max).unwrap().clone();
                    if implem.multiary {
                        implem.clone()
                    } else {
                        SObject::null()
                    }
                } else {
                    let implem = self.func.get(&arity);
                    match implem {
                        Some(res) => res.clone(),
                        None => SObject::null(),
                    }
                }
            }

            // If no max => no implementation
            None => SObject::null(),
        }
    }
}

impl Function {
    pub fn new() -> Function {
        Function {
            multiary: None,
            func: PHashMap::new(),
        }
    }
}

pub fn init() {}

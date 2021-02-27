//! Anonymous Function with multi-arity
use lazy_static::{__Deref, lazy_static};
use std::clone::Clone;
use std::{any::*, fmt::*, hash::*, result::*, sync::*};

// use std::fmt::*;
use intertrait::cast::*;
use intertrait::*;

use super::class::*;
use super::object::*;
use super::phashmap::*;
use super::pvector::*;
use super::rustobj::*;

pub struct SFunction {
    pub multiary: Object,
    pub func: Object, // all implementations
}

castable_to!(SFunction => [sync] TObject, Function);

trait Function {
    fn call(&self, args: &Object) -> Object;

    fn get(&self, arity: &Object) -> Object;
}

impl Function for SFunction {
    fn get(&self, arity: &Object) -> Object {
        match self.multiary {
            Some(max) => {
                if arity > max {
                    let implem = self.func.get(&max).unwrap().clone();
                    if implem.multiary {
                        implem.clone()
                    } else {
                        Object::null()
                    }
                } else {
                    let implem = self.func.get_hash(&arity);
                    match implem {
                        Some(res) => res.clone(),
                        None => Object::null(),
                    }
                }
            }

            // If no max => no implementation
            None => Object::null(),
        }
    }

    fn call(&self, args: Object) -> Object {
        Object::null()
    }
}

impl TObject for SFunction {
    fn get_class<'a>(&'a self) -> Object {
        todo!()
    }

    fn call(&self, name: &str, args: &[Object]) -> Object {
        todo!()
    }

    fn get(&self, name: &str) -> Object {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }
}

impl SFunction {
    pub fn new() -> SFunction {
        SFunction {
            multiary: None,
            func: PHashMap::new(),
        }
    }
}

pub fn init() {}

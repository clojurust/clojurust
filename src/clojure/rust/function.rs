//! Anonymous Function with multi-arity
use crate::clojure::rust::implementation::SImplementation;
use im_rc::hashmap::*;
use im_rc::*;
use lazy_static::{__Deref, lazy_static};
use std::clone::Clone;
use std::{any::*, fmt::*, hash::*, result::*, sync::*};

// use std::fmt::*;
use intertrait::cast::*;
use intertrait::*;

use super::class::*;
use super::implementation::*;
use super::object::*;
use super::pvector::*;
use super::rustobj::*;

pub struct SFunction {
    pub multiary: Option<usize>,
    pub func: HashMap<usize, Object>, // all implementations
}

castable_to!(SFunction => [sync] TObject, Function);

unsafe impl Send for SFunction {}

unsafe impl Sync for SFunction {}

trait Function {
    fn call(&self, args: &Object) -> Object;

    fn get(&self, arity: usize) -> Object;
}

impl Function for SFunction {
    fn get(&self, arity: usize) -> Object {
        let index = arity;
        match self.multiary {
            Some(max) => {
                if arity > max {
                    index = max;
                }
                let implem = self.func.get(&index).clone();
                match implem {
                    Some(o) => {
                        let i = o.cast::<Implementation>();
                        match i {
                            Some(imp) => o.clone(),
                            None => Object::null(),
                        }
                    }
                    None => Object::null(),
                }
            }
            // If no max => no implementation
            None => Object::null(),
        }
    }

    fn call(&self, args: &Object) -> Object {
        Object::null()
    }
}

impl TObject for SFunction {
    fn get_class(&self) -> &SClass {
        todo!()
    }

    fn call(&self, name: usize, args: &[Object]) -> Object {
        todo!()
    }

    fn get(&self, name: usize) -> Object {
        todo!()
    }

    fn to_string(&self) -> &str {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }
}

impl SFunction {
    pub fn new() -> SFunction {
        SFunction {
            multiary: None,
            func: HashMap::new(),
        }
    }

    pub fn init() {}
}

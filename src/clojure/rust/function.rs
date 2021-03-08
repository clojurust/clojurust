//! Anonymous Function with multi-arity
use im::*;
// use lazy_static::{__Deref, lazy_static};
use std::clone::Clone;
// use std::{any::*, fmt::*, hash::*, result::*, sync::*};

// use std::fmt::*;
use intertrait::cast::*;
use intertrait::*;

use super::class::*;
use super::implem_native::*;
use super::object::*;

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
        let mut index = arity;
        match self.multiary {
            Some(max) => {
                if arity > max {
                    index = max;
                }
                let implem = self.func.get(&index).clone();
                match implem {
                    Some(o) => {
                        let i = o.cast::<SImplemNative>();
                        match i {
                            Some(imp) => o.clone(),
                            None => todo!(),
                        }
                    }
                    None => todo!(),
                }
            }
            // If no max => no implementation
            None => todo!(),
        }
    }

    fn call(&self, args: &Object) -> Object {
        Object::null()
    }
}

impl TObject for SFunction {
    fn get_class<'a>(&self) -> &'a SClass {
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

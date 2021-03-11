//! Anonymous Function with multi-arity
//!
//! This is a map of

use im::*;
// use lazy_static::{__Deref, lazy_static};
use std::clone::Clone;
// use std::{any::*, fmt::*, hash::*, result::*, sync::*};

// use std::fmt::*;
// use intertrait::cast::*;
use intertrait::*;

use crate::clojure;
use clojure::rust::class::*;
use clojure::rust::fn_method_native::*;
use clojure::rust::object::*;

pub struct SFunction {
    /// Mark optional arity of multi-arity function.
    pub multiary: Object,
    /// Map of function keyed by arity
    pub func: Object,
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
                        let i = o.cast::<SFnMethodNative>();
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
}

pub unsafe fn init() {
    // only execute one time
    if INIT {
        return;
    }
    INIT = true;

    println!("Function::init");

    // Insures all is initialized
    clojure::rust::object::init();
    clojure::rust::fn_method_native::init();
    clojure::rust::class::init();
}

static mut INIT: bool = false;

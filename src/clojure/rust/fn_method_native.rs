//! # Anonymous Native (Rust) method with a defined arity
//!
//! This is a Method and so use an object as first argument.
//! The method is linked with the `Object`'s `Class` say their protocols.
//!
//! Method can be with multi-arity according to SFunction.multiarity.
//! If this value is Nil, no multi-arity, else the value is the arity
//! of the multi-arity function, which should be the last one.

use std::sync::Arc;

// use intertrait::cast::*;
use intertrait::*;

use crate::clojure;
use clojure::rust::class::*;
use clojure::rust::object::*;

pub type SFnMethodNative = fn(args: &Object) -> Object;

castable_to!(SFnMethodNative => [sync] TObject, FnMethodNative);

pub trait FnMethodNative {
    fn call(&self, args: &Object) -> Object;
}

impl FnMethodNative {
    fn new(function: SFnMethodNative) -> Object {
        Object::new(Arc::new(function))
    }
}

impl FnMethodNative for SFnMethodNative {
    fn call(&self, args: &Object) -> Object {
        let f = self;
        f(args)
    }
}

impl TObject for SFnMethodNative {
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

pub unsafe fn init() {
    // only execute one time
    if INIT {
        return;
    }
    INIT = true;

    println!("FnNative::init");

    // Insures all is initialized
    clojure::rust::object::init();
    clojure::rust::class::init();
    // let c = Keywords::get("clojure.rust.object/Objects");
}

static mut INIT: bool = false;

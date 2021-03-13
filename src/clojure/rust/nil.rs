//! # This will be the Nil virtual Object of the Nil `Object`
//!
//! This wil enable to add `Protocol`s for Nil

use std::sync::Arc;

// use intertrait::cast::*;
use intertrait::*;

use crate::clojure;
use clojure::rust::class::*;
use clojure::rust::object::*;

pub struct Nil;

castable_to!(Nil => [sync] TObject);

impl Nil {
    pub fn new() -> Object {
        Object::new(Arc::new(Nil))
    }
}

impl TObject for Nil {
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

    println!("Nil::init");

    // Insures all is initialized
    clojure::rust::object::init();
    clojure::rust::class::init();
}

static mut INIT: bool = false;

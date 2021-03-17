//! # Prototype defines the methods for `Protocol`s

// use intertrait::cast::*;
use intertrait::*;

use crate::clojure;
use clojure::rust::object::*;

pub struct SPrototype {
    multi_arity: Option<usize>,
}

castable_to!(SPrototype => [sync] TObject, Prototype);

pub trait Prototype {}

impl Prototype {}

impl Prototype for SPrototype {}

impl TObject for SPrototype {
    fn get_class<'a>(&self) -> &'a super::class::SClass {
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

    println!("Prototype::init");

    // Insures all is initialized
    clojure::rust::object::init();
    clojure::rust::class::init();
    clojure::rust::obj_hashset::init();
}

static mut INIT: bool = false;

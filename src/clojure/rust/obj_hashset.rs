// use lazy_static::__Deref;
// use std::{any::*, result::*, sync::*};
// use std::{fmt::*, hash::*};

// use intertrait::cast::*;
use intertrait::*;

use crate::clojure;
use clojure::rust::class::*;
use clojure::rust::object::*;

pub type SObjHashSet = im::hashset::HashSet<Object>;

castable_to!(SObjHashSet => [sync] TObject, ObjHashSet);

impl TObject for SObjHashSet {
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

pub trait ObjHashSet: CastFromSync {}

impl ObjHashSet for SObjHashSet {}

impl ObjHashSet {}

pub unsafe fn init() {
    // only execute one time
    if INIT {
        return;
    }
    INIT = true;

    println!("ObjHashSet::init");

    // Insures all is initialized
    clojure::rust::object::init();
    clojure::rust::class::init();
}

static mut INIT: bool = false;

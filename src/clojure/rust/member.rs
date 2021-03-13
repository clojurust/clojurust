//! # Access to `Member`s getters and setters of `Objects` inner structure.
//!
//! This will be part of the `Class` store.

// use intertrait::cast::*;
use intertrait::*;

use crate::clojure;
use clojure::rust::class::*;
use clojure::rust::object::*;

pub struct SMember {
    name: usize,
    class: usize,
    getter: Object,
    setter: Object,
}

impl SMember {
    pub fn new(
        name: usize,
        class: usize,
        getter: Object, // fn_native ?
        setter: Object, // fn_native ?
    ) -> SMember {
        SMember {
            name,
            class,
            getter,
            setter,
        }
    }
}

castable_to!(SMember => [sync] TObject, Member);

pub trait Member {}

impl Member {}

impl Member for SMember {}

impl TObject for SMember {
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

    println!("Member::init");

    // Insures all is initialized
    clojure::rust::object::init();
    clojure::rust::class::init();
}

static mut INIT: bool = false;

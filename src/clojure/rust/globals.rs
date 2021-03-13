//! # Stores for Rust `Objects` identified by unique string name
//!
//! There will be multiple stores:
//! * `Class`es
//! * `Prototype`s
//! * `Class`es and `Prototype`s' `Function`s.
//! * `Object`s' `Member`s' getter and setters.

// use std::sync::*;
use lazy_static::lazy_static;

// use intertrait::cast::*;
use intertrait::*;

use crate::clojure;
use clojure::rust::class::*;
use clojure::rust::obj_vector::*;
use clojure::rust::object::*;
use clojure::rust::unique::*;

pub struct SGlobals {
    pub id: Object,  // SUnique
    pub obj: Object, // SObjVector
}

castable_to!(SGlobals => [sync] TObject, Globals);

pub trait Globals {
    fn update_object(&mut self, index: usize, value: &Object) -> (usize, Object);

    fn get_obj_by_id(&self, index: usize) -> Object;
}

impl SGlobals {
    pub fn new() -> Object {
        Object::new::<SGlobals>(SGlobals {
            id: Object::new::<SUnique>(SUnique::new()),
            obj: Object::new::<SObjVector>(SObjVector::default()),
        })
    }
}

impl Globals for SGlobals {
    fn update_object(&mut self, index: String, value: &Object) -> (usize, Object) {
        let v = self;
        let b =
            v.id.clone()
                .inn_mut::<SUnique>()
                .get_id(index, value.clone());
        Object::new::<SGlobals>(SGlobals {
            id: self.id,
            obj: v,
        })
    }

    fn get_obj_by_id(&self, index: usize) -> Object {
        self.obj.get(index).expect("TODO object not found").clone()
    }
}

impl TObject for SGlobals {
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

    println!("Globals::init");

    // Insures all is initialized
    clojure::rust::object::init();
    clojure::rust::fn_method_native::init();
    clojure::rust::class::init();
}

static mut INIT: bool = false;

lazy_static! {
    pub static ref RUSTOBJ: SGlobals = SGlobals::new();
}

#[test]
fn test_rust_obj() {}

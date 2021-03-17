//! # Stores for Rust `Objects` identified by unique string name
//!
//! There will be multiple stores:
//! * `Class`es
//! * `Prototype`s
//! * `Class`es and `Prototype`s' `Function`s.
//! * `Object`s' `Member`s' getter and setters.

use lazy_static::lazy_static;
use std::sync::*;

// use intertrait::cast::*;

use crate::use_obj;

use_obj! {
    clojure::rust::object;
    clojure::rust::class;
    clojure::rust::obj_vector;
    clojure::rust::unique;
}

castable_to!(SGlobals => [sync] TObject, Globals);

init_obj! {
    Globals {
        clojure::rust::object::init();
        clojure::rust::class::init();
        clojure::rust::obj_vector::init();
        clojure::rust::unique::init();
    }
}

pub struct SGlobals {
    pub id: Object,  // SUnique
    pub obj: Object, // SObjVector
}

castable_to!(SGlobals => [sync] TObject, Globals);

pub trait Globals {
    fn update_object(&mut self, index: usize, value: &Object) -> (usize, Object);

    fn get_obj_by_id(&self, index: usize) -> Object;

    fn get_obj_by_name(&self, index: &str) -> Object;
}

impl SGlobals {
    pub fn new() -> Object {
        Object::new(Arc::new(SGlobals {
            id: SUnique::new(),
            obj: SObjVector::new(),
        }))
    }
}

impl Globals for SGlobals {
    fn update_object(&mut self, index: String, value: &Object) -> (usize, Object) {
        let v = self;
        let b = v.id.clone().inn_mut::<SUnique>().get(index, value.clone());
        Object::new(Arc::new(SGlobals {
            id: self.id,
            obj: v,
        }));
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

lazy_static! {
    pub static ref RUSTOBJ: SGlobals = SGlobals::new();
}

#[test]
fn test_rust_obj() {}

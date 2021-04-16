//! # Stores for Rust `Objects` identified by unique string name
//!
//! There will be multiple stores:
//! * `Class`es
//! * `Prototype`s
//! * `Class`es and `Prototype`s' `Function`s.
//! * `Object`s' `Member`s' getter and setters.

use std::{fmt::*, sync::*};

use lazy_static::lazy_static;

// use intertrait::cast::*;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::Class;
    clojure::lang::PersistentVector;
    clojure::rust::Unique;
}

castable_to!(SGlobals => [sync] TObject, Globals);

init_obj! {
    Globals {
        clojure::rust::Object::init();
        clojure::rust::Class::init();
        clojure::lang::PersistentVector::init();
        clojure::rust::Unique::init();
    }
}

#[derive(Debug)]
pub struct SGlobals {
    pub unique_name: Object, // SUnique
    pub obj_vect: Object,  // SObjVector
}

pub trait Globals: CastFromSync {
    fn update_object(&mut self, name: &str, value: &Object) -> Option<(usize, Object)>;

    fn add_object(&mut self, name: &str, value: &Object) -> usize;

    fn get_obj_by_id(&self, index: usize) -> Object;

    fn get_obj_by_name(&self, name: &str) -> Object;

    fn get_id_for_name(&self, name: &str) -> Option<usize>;
}

impl SGlobals {
    fn new() -> Object {
        new_obj!(SGlobals::default())
    }
}

use crate::new_obj;

impl Globals for SGlobals {
    fn update_object(&mut self, name: &str, value: &Object) -> Option<(usize, Object)> {
        todo!()
    }

    fn add_object(&mut self, name: &str, value: &Object) -> usize {
        todo!()
    }

    fn get_obj_by_id(&self, index: usize) -> Object {
        todo!()
    }

    fn get_obj_by_name(&self, index: &str) -> Object {
        todo!()
    }

    fn get_id_for_name(&self, name: &str) -> Option<usize> {
        todo!()
    }
}

impl Display for SGlobals {
    /// Return string representation of
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Globals")
    }
}

impl TObject for SGlobals {
    fn get_class<'a>(&self) -> &'a SClass {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }
}

impl Default for SGlobals {
    fn default() -> Self {
        SGlobals {
            unique_name: SUnique::new(),
            obj_vect: new_obj!(SPersistentVector::default()),
        }
    }
}

lazy_static! {
    pub static ref CLASSES: Object = Object { inner: None };
    pub static ref PROTOCOLS: Object = Object { inner: None };
}

//! # This will be the Nil virtual Object of the Nil `Object`
//!
//! This wil enable to add `Protocol`s for Nil

// use intertrait::cast::*;

use crate::use_obj;

use_obj! {
    clojure::rust::object;
    clojure::rust::class;
}

castable_to!(Nil => [sync] TObject, TNil);

init_obj! {
    Nil {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

#[derive(Debug)]
pub struct Nil {}

pub trait TNil: CastFromSync {}

impl Nil {
    pub fn new() -> Object {
        Object::new(None)
    }
}

impl TNil for Nil {}

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

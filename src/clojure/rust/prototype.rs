//! # `Prototype` defines the methods of `Protocol`s

// use intertrait::cast::*;

use crate::use_obj;

use_obj! {
    clojure::rust::object;
    clojure::rust::class;
}

castable_to!(SPrototype => [sync] TObject, Prototype);

init_obj! {
    Prototype {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

pub struct SPrototype {
    multi_arity: Option<usize>,
}

pub trait Prototype: CastFromSync {}

impl Prototype {}

impl Prototype for SPrototype {}

impl TObject for SPrototype {
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

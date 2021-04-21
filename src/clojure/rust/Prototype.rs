//! # `Prototype` defines the methods of `Protocol`s

// use intertrait::cast::*;

use std::fmt::*;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::Class;
}

use intertrait::*;
castable_to!(SPrototype => [sync] IObject, Prototype);

init_obj! {
    Prototype {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::Class::init();
    }
}

#[derive(Debug)]
pub struct SPrototype {
    multi_arity: Option<usize>,
}

pub trait Prototype: CastFromSync {}

impl Prototype {}

impl Prototype for SPrototype {}

impl IObject for SPrototype {
    fn getClass<'a>(&self) -> &'a SClass {
        todo!()
    }

    fn hashCode(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }

    fn toString(&self) -> usize {
        todo!()
    }
}

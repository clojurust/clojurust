//! # `Prototype` defines the methods of `Protocol`s

// use intertrait::cast::*;

use std::fmt::*;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::Class;
}

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

impl Display for SPrototype {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "^Prototype {:?}", self.multi_arity)
    }
}

impl IObject for SPrototype {
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

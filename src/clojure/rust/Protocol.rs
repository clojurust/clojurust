//! # Protocol is the eqivalent of `traits`.
//!
//! `Protocol` defines the templates for the functions of the trait
//! for the library usage.

// use intertrait::cast::*;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::Class;
    clojure::lang::PersistentHashSet;
}

use intertrait::*;
castable_to!(SProtocol => [sync] IObject, Protocol);

init_obj! {
    Protocols {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::Class::init();
        clojure::lang::PersistentHashSet::init();
    }
}

pub struct SProtocol {
    inner: SPersistentHashSet
} 

impl IObject for SProtocol {
    #[allow(non_snake_case)]
    fn getClass<'a>(&self) -> &'a SClass {
        todo!()
    }

    #[allow(non_snake_case)]
    fn hashCode(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }

    #[allow(non_snake_case)]
    fn toString(&self) -> usize {
        todo!()
    }
}

pub trait Protocol: IObject {}

impl Protocol {}

impl Protocol for SProtocol {}

impl SProtocol {}

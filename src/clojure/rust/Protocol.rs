//! # Protocol is the eqivalent of `traits`.
//!
//! `Protocol` defines the templates for the functions of the trait
//! for the library usage.

// use intertrait::cast::*;

use std::fmt::*;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::Class;
    clojure::lang::PersistentHashSet;
}

castable_to!(SProtocol => [sync] IObject, Protocol);

init_obj! {
    Protocols {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::Class::init();
        clojure::lang::PersistentHashSet::init();
    }
}

pub type SProtocol = SPersistentHashSet;

// #[derive(Debug)]
// struct SProtocol {
//     /// This is the template functions of the `Prototype`.
//     /// TODO
//     template: Object, // SPersistentHashSet of Prototype
// }

impl Display for SProtocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "^Protocol {:?}", self.template)
    }
}

impl IObject for SProtocol {
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

pub trait Protocol: IObject {}

impl Protocol {}

impl Protocol for SProtocol {}

impl SProtocol {}

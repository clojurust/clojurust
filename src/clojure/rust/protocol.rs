//! # Protocol is the eqivalent of `traits`.
//!
//! `Protocol` defines the templates for the functions of the trait
//! for the library usage.

// use intertrait::cast::*;

use crate::use_obj;

use_obj! {
    clojure::rust::object;
    clojure::rust::class;
}

castable_to!(SProtocol => [sync] TObject, Protocol);

init_obj! {
    Protocols {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

struct SProtocol {
    /// This is the template functions of the `Prototype`.
    /// TODO
    template: Object, // SObjHashSet of Prototype
}

castable_to!(SProtocol => [sync] TObject, Protocol);

impl TObject for SProtocol {
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

pub trait Protocol: CastFromSync {}

impl Protocol {}

impl Protocol for SProtocol {}

impl SProtocol {}

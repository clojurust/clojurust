/// IObject

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

init_obj! {
    IObject {
        clojure::rust::Class::init();
        clojure::rust::Object::init();
    }
}

/// `IObject` `Protocol` for all defined `Object`s
///
///
use intertrait::*;
pub trait IObject: CastFromSync  {
    /// Return `Class` of `Object`
    fn getClass<'a>(&self) -> &'a SClass;

    fn hashCode(&self) -> usize;

    fn toString(&self) -> String;

    fn equals(&self, other: &Object) -> bool;
}

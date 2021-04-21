/// IObject
use crate::*;

use_obj! {
    clojure::rust::Class;
    clojure::rust::Object;
}

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

    fn toString(&self) -> usize;

    fn equals(&self, other: &Object) -> bool;
}

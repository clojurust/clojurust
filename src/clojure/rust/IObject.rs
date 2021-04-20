/// IObject
use std::fmt::*;

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
pub trait IObject: Debug + Display + CastFromSync  {
    /// Return `Class` of `Object`
    fn get_class<'a>(&self) -> &'a SClass;

    fn get_hash(&self) -> usize;

    fn equals(&self, other: &Object) -> bool;
}

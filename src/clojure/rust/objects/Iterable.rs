/// Iterable

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

init_obj! {
    Iterator {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait Iterable: IObject {
    /// Iterable -> Iterator
    fn iterator(&self) -> ObjResult<Object>;
}
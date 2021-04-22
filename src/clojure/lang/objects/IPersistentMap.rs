//! Protocol IPersistentMap

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    IPersistentMap {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Counted::init();
        clojure::rust::Iterable::init();
        clojure::rust::Associative::init();
    }
}

pub trait IPersistentMap: IObject + Associative + Iterable + Counted {
    fn assoc(&self, key: Object, val: Object) -> ObjResult<Object>;
    fn assocEx(&self, key: Object, val: Object) -> ObjResult<Object>;
    fn without(&self, key: Object) -> ObjResult<Object>;
}
//! Protocol IPersistentCollection

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjResult::*;
use clojure::lang::Sequable::*;
use crate::*;

use_obj! {
    clojure::lang::Sequable;
    clojure::rust::Object;
    clojure::rust::ObjResult;
}

init_obj! {
    IPersistentCollection {
        clojure::lang::Sequable::init();
        clojure::rust::Object::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait IPersistentCollection: IObject + Sequable {
    fn cons(&self, o: &Object) -> ObjResult<Object>;
    fn count(&self) -> ObjResult<usize>;
    fn empty(&self) -> ObjResult<Object>;
    fn equiv(&self, o: Object) -> ObjResult<bool>;
}
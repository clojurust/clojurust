//! Protocol IPersistentCollection

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    IPersistentCollection {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::lang::Sequable::init();
    }
}

pub trait IPersistentCollection: IObject + Sequable {
    fn cons(&self, o: &Object) -> ObjResult<Object>;
    fn count(&self) -> ObjResult<usize>;
    fn empty(&self) -> ObjResult<Object>;
    fn equiv(&self, o: Object) -> ObjResult<bool>;
}
//! Protocol IPersistentCollection

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;
use clojure::lang::Sequable::*;
use crate::*;

use_obj! {
    clojure::lang::Sequable;
    clojure::rust::Object;
    clojure::rust::ObjError;
}

init_obj! {
    IPersistentCollection {
        clojure::lang::Sequable::init();
        clojure::rust::Object::init();
        clojure::rust::ObjError::init();
    }
}

pub trait IPersistentCollection: TObject + Sequable {
    fn cons(&self, o: &Object) -> ObjResult<Object>;
    fn count(&self) -> ObjResult<usize>;
    fn empty(&self) -> ObjResult<Object>;
    fn equiv(&self, o: Object) -> ObjResult<bool>;
}
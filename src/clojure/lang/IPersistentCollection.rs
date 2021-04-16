//! Protocol IPersistentCollection

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;
use clojure::lang::Sequable::*;

pub trait IPersistentCollection: Sequable {
    fn cons(&self, o: &Object) -> ObjResult<&'_ IPersistentCollection>;
    fn count(&self) -> ObjResult<usize>;
    fn empty(&self) -> ObjResult<&'_ IPersistentCollection>;
    fn equiv(&self, o: Object) -> ObjResult<bool>;
}
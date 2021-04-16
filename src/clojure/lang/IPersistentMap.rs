//! Protocol IPersistentMap
use crate::clojure;
use clojure::rust::Counted::*;
use clojure::rust::Iterable::*;
use clojure::rust::Associative::*;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;

pub trait IPersistentMap: TObject + Associative + Iterable + Counted {
    fn assoc(&self, key: Object, val: Object) -> ObjResult<&'_ IPersistentMap>;
    fn assocEx(&self, key: Object, val: Object) -> ObjResult<&'_ IPersistentMap>;
    fn without(&self, key: Object) -> ObjResult<&'_ IPersistentMap>;
}
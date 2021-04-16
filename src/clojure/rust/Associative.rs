//! Protocol Associative

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;
use clojure::lang::IMapEntry::*;

pub trait Associative {
    fn assoc(&self, key: &Object, value: &Object) -> ObjResult<&Associative>;
    fn containsKey(&self, key: &Object) -> ObjResult<bool>;
    fn entryAt(&self, key: &Object) -> ObjResult<&IMapEntry>;
}
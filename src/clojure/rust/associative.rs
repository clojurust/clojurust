//! Protocol Associative

use crate::clojure;
use clojure::rust::object::*;
use clojure::rust::obj_error::*;
use clojure::lang::i_map_entry::*;

pub trait Associative {
    fn assoc(&self, key: &Object, value: &Object) -> ObjResult<&Associative>;
    fn containsKey(&self, key: &Object) -> ObjResult<bool>;
    fn entryAt(&self, key: &Object) -> ObjResult<&IMapEntry>;
}
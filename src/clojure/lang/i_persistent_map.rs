//! Protocol IPersistentMap
use core::iter::*;

use crate::clojure;
use clojure::rust::counted::*;
use clojure::rust::associative::*;
use clojure::rust::object::*;
use clojure::rust::obj_error::*;

pub trait IPersistentMap: TObject + IntoIterator + Associative + Counted {
    fn assoc(&self, key: Object, val: Object) -> ObjResult<&'_ IPersistentMap>;
    fn assocEx(&self, key: Object, val: Object) -> ObjResult<&'_ IPersistentMap>;
    fn without(&self, key: Object) -> ObjResult<&'_ IPersistentMap>;
}
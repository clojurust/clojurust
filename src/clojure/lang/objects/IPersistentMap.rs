//! Protocol IPersistentMap

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait IPersistentMap: IObject + Associative + Iterable + Counted {
    fn assoc(&self, key: Object, val: Object) -> ObjResult<Object>;
    fn assocEx(&self, key: Object, val: Object) -> ObjResult<Object>;
    fn without(&self, key: Object) -> ObjResult<Object>;
}
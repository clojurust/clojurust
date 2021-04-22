//! Protocol IPersistentMap

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

pub trait IPersistentSet: IObject + IPersistentCollection + Counted {
    // IPersistentSet -> Object -> IPersistentSet
    fn disjoin(&self, key: Object) -> ObjResult<Object>;

    // IPersistentSet -> Object -> bool
    fn assocEx(&self, key: Object) -> ObjResult<bool>;

    // IPersistentSet -> Object -> Object
    fn get(&self, key: Object) -> ObjResult<Object>;
}
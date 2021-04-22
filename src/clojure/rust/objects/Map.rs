//! Protocol Map

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

pub trait Map: IObject + IPersistentCollection + Counted {
    // 


}

pub trait Entry: IObject {
    // Entry -> Object (key)
    fn getKey(&self) -> ObjResult<Object>;

    // Entry -> Object (key)
    fn getValue(&self) -> ObjResult<Object>;
}
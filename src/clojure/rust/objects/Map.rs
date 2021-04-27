//! Protocol Map

use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait Map: IObject+IPersistentCollection+Counted {
    //
}

pub trait Entry: IObject {
    // Entry -> Object (key)
    fn getKey(&self) -> ObjResult<Object>;

    // Entry -> Object (key)
    fn getValue(&self) -> ObjResult<Object>;
}

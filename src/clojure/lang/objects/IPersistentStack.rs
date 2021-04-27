//! Protocol IPersistentStack

use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait IPersistentStack: IObject+IPersistentCollection {
    fn peek(&self) -> ObjResult<Object>;

    fn pop(&self) -> ObjResult<Object>;
}

//! Protocol IPersistentStack

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

pub trait IPersistentStack: IObject + IPersistentCollection {
    fn peek(&self) -> ObjResult<Object>;
    fn pop(&self) -> ObjResult<Object>;
}
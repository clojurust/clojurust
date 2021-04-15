//! Protocol IPersistentStack

use crate::clojure;
use clojure::rust::object::*;
use clojure::rust::obj_error::*;
use clojure::lang::i_persistent_collection::*;

pub trait IPersistentStack: TObject + IPersistentCollection {
    fn peek(&self) -> ObjResult<Object>;
    fn pop(&self) -> ObjResult<&'_ IPersistentStack>;
}
//! Protocol IPersistentStack

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;
use clojure::lang::IPersistentCollection::*;

pub trait IPersistentStack: TObject + IPersistentCollection {
    fn peek(&self) -> ObjResult<Object>;
    fn pop(&self) -> ObjResult<&'_ IPersistentStack>;
}
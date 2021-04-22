//! Protocol IPersistentStack

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    IPersistentStack {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::lang::IPersistentCollection::init();
    }
}

pub trait IPersistentStack: IObject + IPersistentCollection {
    fn peek(&self) -> ObjResult<Object>;
    fn pop(&self) -> ObjResult<Object>;
}
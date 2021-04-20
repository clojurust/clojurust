//! Protocol IPersistentStack

use crate::*;

use_obj! {
    clojure::lang::IPersistentCollection;
    clojure::rust::Object;
    clojure::rust::ObjResult;
}

init_obj! {
    Runnable {
        clojure::lang::IPersistentCollection::init();
        clojure::rust::Object::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait IPersistentStack: IObject + IPersistentCollection {
    fn peek(&self) -> ObjResult<Object>;
    fn pop(&self) -> ObjResult<Object>;
}
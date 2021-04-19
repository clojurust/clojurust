//! Protocol IPersistentStack

use crate::*;

use_obj! {
    clojure::lang::IPersistentCollection;
    clojure::rust::Object;
    clojure::rust::ObjError;
}

init_obj! {
    Runnable {
        clojure::lang::IPersistentCollection::init();
        clojure::rust::Object::init();
        clojure::rust::ObjError::init();
    }
}

pub trait IPersistentStack: TObject + IPersistentCollection {
    fn peek(&self) -> ObjResult<Object>;
    fn pop(&self) -> ObjResult<Object>;
}
//! Protocol IPersistentVector

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    IPersistentVector {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Indexed::init();
        clojure::rust::Reversible::init();
        clojure::rust::Associative::init();
        clojure::lang::IPersistentStack::init();
        }
}

pub trait IPersistentVector: IObject + Associative 
        + IPersistentStack + Reversible + Indexed {
    fn assocN(&self, i: usize, val: &Object) -> ObjResult<Object>;
    fn cons(&self, o: Object) -> ObjResult<Object>;
    fn length(&self) -> ObjResult<usize>;
}
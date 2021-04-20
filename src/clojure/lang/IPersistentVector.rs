//! Protocol IPersistentVector

use crate::clojure;
use clojure::rust::Indexed::*;
use clojure::rust::Reversible::*;
use clojure::rust::Associative::*;
use clojure::rust::Object::*;
use clojure::rust::ObjResult::*;
use clojure::lang::IPersistentStack::*;

use crate::*;

use_obj! {
    clojure::rust::Indexed;
    clojure::rust::Reversible;
    clojure::rust::Associative;
    clojure::rust::Object;
    clojure::rust::ObjResult;
    clojure::lang::IPersistentStack;
}

init_obj! {
    Runnable {
        clojure::rust::Indexed::init();
        clojure::rust::Reversible::init();
        clojure::rust::Associative::init();
        clojure::rust::Object::init();
        clojure::rust::ObjResult::init();
        clojure::lang::IPersistentStack::init();
    }
}

pub trait IPersistentVector: IObject + Associative 
        + IPersistentStack + Reversible + Indexed {
    fn assocN(&self, i: usize, val: &Object) -> ObjResult<Object>;
    fn cons(&self, o: Object) -> ObjResult<Object>;
    fn length(&self) -> ObjResult<usize>;
}
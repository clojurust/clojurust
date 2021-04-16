//! Protocol IPersistentVector

use crate::clojure;
use clojure::rust::Indexed::*;
use clojure::rust::Reversible::*;
use clojure::rust::Associative::*;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;
use clojure::lang::IPersistentStack::*;

pub trait IPersistentVector: TObject + Associative 
        + IPersistentStack + Reversible + Indexed {
    fn assocN(&self, i: usize, val: &Object) -> ObjResult<&'_ IPersistentVector>;
    fn cons(&self, o: Object) -> ObjResult<&'_ IPersistentVector>;
    fn length(&self) -> ObjResult<usize>;
}
//! Protocol IPersistentVector

use crate::clojure;
use clojure::rust::indexed::*;
use clojure::rust::reversible::*;
use clojure::rust::associative::*;
use clojure::rust::object::*;
use clojure::rust::obj_error::*;
use clojure::lang::i_persistent_stack::*;

pub trait IPersistentVector: TObject + Associative 
        + IPersistentStack + Reversible + Indexed {
    fn assocN(&self, i: usize, val: &Object) -> ObjResult<&'_ IPersistentVector>;
    fn cons(&self, o: Object) -> ObjResult<&'_ IPersistentVector>;
    fn length(&self) -> ObjResult<usize>;
}
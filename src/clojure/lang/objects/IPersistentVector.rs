//! Protocol IPersistentVector

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

pub trait IPersistentVector: IObject + Associative 
        + IPersistentStack + Reversible + Indexed {
    fn assocN(&self, i: usize, val: &Object) -> ObjResult<Object>;
    fn cons(&self, o: Object) -> ObjResult<Object>;
    fn length(&self) -> ObjResult<usize>;
}
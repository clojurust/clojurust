//! Protocol IPersistentVector

use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait IPersistentVector:
    IObject+Associative+IPersistentStack+Reversible+Indexed
{
    fn assocN(
        &self,
        i: usize,
        val: &Object,
    ) -> ObjResult<Object>;

    fn cons(
        &self,
        o: Object,
    ) -> ObjResult<Object>;

    fn length(&self) -> ObjResult<usize>;
}

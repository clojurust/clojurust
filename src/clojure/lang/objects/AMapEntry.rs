//! Protocol AMapEntry

use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait AMapEntry: IObject+APersistentVector+IMapEntry
{
    fn assocN(
        &self,
        i: usize,
        val: &Object,
    ) -> ObjResult<&'_ IPersistentVector>;
    fn cons(
        &self,
        o: &Object,
    ) -> ObjResult<&'_ IPersistentVector>;
    fn count(&self) -> ObjResult<usize>;
    fn empty(&self) -> ObjResult<&'_ IPersistentCollection>;
    fn nth(
        &self,
        i: usize,
    ) -> ObjResult<Object>;
    fn pop(&self) -> ObjResult<&'_ IPersistentStack>;
    fn seq(&self) -> ObjResult<&'_ ISeq>;
    fn setValue(value: Object) -> ObjResult<Object>;
}

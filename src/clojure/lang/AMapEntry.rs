//! Protocol AMapEntry

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;
use clojure::lang::APersistentVector::*;
use clojure::lang::IPersistentVector::*;
use clojure::lang::IPersistentStack::*;
use clojure::lang::IPersistentCollection::*;
use clojure::lang::ISeq::*;
use clojure::lang::IMapEntry::*;

pub trait AMapEntry : TObject + APersistentVector + IMapEntry {
    fn assocN(&self, i: usize, val: &Object) -> ObjResult<&'_ IPersistentVector>; 
    fn cons(&self, o: &Object) -> ObjResult<&'_ IPersistentVector>;
    fn count(&self) -> ObjResult<usize>;
    fn empty(&self) -> ObjResult<&'_ IPersistentCollection>; 
    fn nth(&self, i: usize) -> ObjResult<Object>;
    fn pop(&self) -> ObjResult<&'_ IPersistentStack>; 
    fn seq(&self) -> ObjResult<&'_ ISeq>; 
    fn setValue(value: Object) -> ObjResult<Object>; 
}
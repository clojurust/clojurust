//! Protocol AMapEntry

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    AMapEntry {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::lang::APersistentVector::init();
        clojure::lang::IPersistentVector::init();
        clojure::lang::IPersistentStack::init();
        clojure::lang::IPersistentCollection::init();
        clojure::lang::ISeq::init();
        clojure::lang::IMapEntry::init();
    }
}

pub trait AMapEntry : IObject + APersistentVector + IMapEntry {
    fn assocN(&self, i: usize, val: &Object) -> ObjResult<&'_ IPersistentVector>; 
    fn cons(&self, o: &Object) -> ObjResult<&'_ IPersistentVector>;
    fn count(&self) -> ObjResult<usize>;
    fn empty(&self) -> ObjResult<&'_ IPersistentCollection>; 
    fn nth(&self, i: usize) -> ObjResult<Object>;
    fn pop(&self) -> ObjResult<&'_ IPersistentStack>; 
    fn seq(&self) -> ObjResult<&'_ ISeq>; 
    fn setValue(value: Object) -> ObjResult<Object>; 
}
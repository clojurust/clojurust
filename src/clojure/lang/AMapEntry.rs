//! Protocol AMapEntry

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::ObjError;
    clojure::lang::APersistentVector;
    clojure::lang::IPersistentVector;
    clojure::lang::IPersistentStack;
    clojure::lang::IPersistentCollection;
    clojure::lang::ISeq;
    clojure::lang::IMapEntry;
    }

init_obj! {
    AMapEntry {
        clojure::rust::Object::init();
        clojure::rust::ObjError::init();
        clojure::lang::APersistentVector::init();
        clojure::lang::IPersistentVector::init();
        clojure::lang::IPersistentStack::init();
        clojure::lang::IPersistentCollection::init();
        clojure::lang::ISeq::init();
        clojure::lang::IMapEntry::init();
    }
}

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
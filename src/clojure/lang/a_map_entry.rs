//! Protocol AMapEntry

use crate::clojure;
use clojure::rust::object::*;
use clojure::rust::obj_error::*;
use clojure::lang::a_persistent_vector::*;
use clojure::lang::i_persistent_vector::*;
use clojure::lang::i_persistent_stack::*;
use clojure::lang::i_persistent_collection::*;
use clojure::lang::i_seq::*;
use clojure::lang::i_map_entry::*;

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
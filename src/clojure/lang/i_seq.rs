//! Protocol ISeq

use crate::clojure;
use clojure::rust::obj_error::*;
use clojure::rust::object::*;
use clojure::lang::i_persistent_collection::*;
use clojure::lang::sequable::*;

pub trait ISeq: TObject + Sequable + IPersistentCollection {
    fn cons(&self, o: &Object) -> ObjResult<&'_ ISeq>;
}
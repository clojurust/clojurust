//! Protocol ISeq

use crate::clojure;
use clojure::rust::ObjError::*;
use clojure::rust::Object::*;
use clojure::lang::IPersistentCollection::*;
use clojure::lang::Sequable::*;

pub trait ISeq: TObject + Sequable + IPersistentCollection {
    fn cons(&self, o: &Object) -> ObjResult<&'_ ISeq>;
}
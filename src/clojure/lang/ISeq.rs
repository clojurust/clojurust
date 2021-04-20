//! Protocol ISeq

use crate::clojure;
use clojure::rust::ObjResult::*;
use clojure::rust::Object::*;
use clojure::lang::IPersistentCollection::*;
use clojure::lang::Sequable::*;

pub trait ISeq: IObject + Sequable + IPersistentCollection {
    fn cons(&self, o: &Object) -> ObjResult<&'_ ISeq>;
}
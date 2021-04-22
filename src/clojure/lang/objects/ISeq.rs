//! Protocol ISeq

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    ISeq {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::lang::IPersistentCollection::init();
        clojure::lang::Sequable::init();
        }
}

pub trait ISeq: IObject + Sequable + IPersistentCollection {
    /// ISeq -> Object -> ISeq
    fn cons(&self, o: &Object) -> ObjResult<Object>;
}
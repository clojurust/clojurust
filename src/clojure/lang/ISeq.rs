//! Protocol ISeq

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
    clojure::lang::IPersistentCollection;
    clojure::lang::Sequable;
    }

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
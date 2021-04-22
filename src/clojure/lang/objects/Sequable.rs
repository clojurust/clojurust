//! Protocol sequable

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    PersistentHashSet {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::lang::APersistentSet::init();
        clojure::lang::PersistentHashSet::init();
    }
}

pub trait Sequable: IObject {
    /// Sequable -> ISeq
    fn seq(&self) -> ObjResult<Object>;
}
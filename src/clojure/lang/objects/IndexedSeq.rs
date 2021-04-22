
use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    IndexedSeq {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::lang::IPersistentCollection::init();
        clojure::lang::Sequable::init();
        clojure::lang::Sequential::init();
        clojure::rust::Counted::init();
        clojure::lang::ISeq::init();
    }
}


pub trait IndexedSeq: IObject + Counted + IPersistentCollection + ISeq 
        + Sequable + Sequential {
    fn index(&self) -> ObjResult<usize>;
}
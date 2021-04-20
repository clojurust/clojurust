
use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
    clojure::lang::IPersistentCollection;
    clojure::lang::Sequable;
    clojure::lang::Sequential;
    clojure::rust::Counted;
    clojure::lang::ISeq;
}

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
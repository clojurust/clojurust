use crate::clojure;
// use clojure::rust::Object::*;
use clojure::lang::IPersistentCollection::*;
use clojure::lang::Sequable::*;
use clojure::lang::Sequential::*;
use clojure::rust::Counted::*;
use clojure::rust::Object::*;
use clojure::lang::ISeq::*;

use crate::*;

use_obj! {
    clojure::lang::IPersistentCollection;
    clojure::lang::Sequable;
    clojure::lang::Sequential;
    clojure::rust::Counted;
    clojure::rust::Object;
    clojure::rust::ObjError;
    clojure::lang::ISeq;
}

init_obj! {
    IndexedSeq {
        clojure::lang::IPersistentCollection::init();
        clojure::lang::Sequable::init();
        clojure::lang::Sequential::init();
        clojure::rust::Counted::init();
        clojure::rust::Object::init();
        clojure::rust::ObjError::init();
        clojure::lang::ISeq::init();
    }
}


pub trait IndexedSeq: TObject + Counted + IPersistentCollection + ISeq 
        + Sequable + Sequential {
    fn index(&self) -> ObjResult<usize>;
}
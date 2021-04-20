//! Protocol IPersistentMap

use crate::*;

use_obj! {
    clojure::rust::Counted;
    clojure::rust::Iterable;
    clojure::rust::Associative;
    clojure::rust::Object;
    clojure::rust::ObjResult;
}

init_obj! {
    Runnable {
        clojure::rust::Counted::init();
        clojure::rust::Iterable::init();
        clojure::rust::Associative::init();
        clojure::rust::Object::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait IPersistentMap: IObject + Associative + Iterable + Counted {
    fn assoc(&self, key: Object, val: Object) -> ObjResult<Object>;
    fn assocEx(&self, key: Object, val: Object) -> ObjResult<Object>;
    fn without(&self, key: Object) -> ObjResult<Object>;
}

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
    clojure::rust::Iterator;
}

init_obj! {
    PersistentHashSet {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Iterator::init();
    }
}

pub trait IMapIterable: IObject + Iterator {
    /// IMapIterable -> Iterator
    fn keyIterator(&self) -> ObjResult<Object>; 

    /// IMapIterable -> Iterator
    fn valIterator(&self) -> ObjResult<Object>; 
}
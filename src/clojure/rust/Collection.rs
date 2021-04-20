
use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
    clojure::rust::Iterable;
}

init_obj! {
    Collection {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Iterable::init();
    }
}

pub trait Collection: IObject + Iterable {
    
}

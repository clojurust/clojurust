/// Comparator

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

init_obj! {
    Comparator {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait Comparator: IObject {
    /// AFunction -> Object -> Object -> int
    fn compare(o1: Object, o2: Object) -> i8;
}

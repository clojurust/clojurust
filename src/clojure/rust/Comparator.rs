/// Comparator

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::ObjError;
}

init_obj! {
    Comparator {
        clojure::rust::Object::init();
        clojure::rust::ObjError::init();
    }
}

pub trait Comparator: TObject {
    /// AFunction -> Object -> Object -> int
    fn compare(o1: Object, o2: Object) -> i8;
}

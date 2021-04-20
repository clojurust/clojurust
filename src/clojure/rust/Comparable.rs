//! Protocol Comparable

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
}

init_obj! {
    Comparable {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait Comparable: IObject {
    /// Compare with object
    ///
    /// result < 0 iff self < o 
    /// result > 0 iff self > o 
    /// result 0 0 iff self = o
    /// throw NullPointerException iff o = nil
    /// throw ClassCastException iff o and self are not comparable
    #[allow(non_snake_case)]
    fn compareTo(&self, o: &Object) -> ObjResult<i8>;
}
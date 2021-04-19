// use intertrait::cast::*;
use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::ObjError;
    clojure::rust::Comparator;
    clojure::rust::Serializable;
    clojure::lang::AFn;
    clojure::lang::IObj;
}

init_obj! {
    AFunction {
        clojure::rust::Object::init();
        clojure::rust::ObjError::init();
        clojure::rust::Comparator::init();
        clojure::rust::Serializable::init();
        clojure::lang::AFn::init();
        clojure::lang::IObj::init();
    }
}

pub trait AFunction: AFn + IObj + Comparator + Serializable {
    /// AFunction -> MethodImplCache
    fn __methodImplCache(&self) -> MethodImplCache;
}


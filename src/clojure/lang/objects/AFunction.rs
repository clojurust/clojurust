// use intertrait::cast::*;
use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    AFunction {
        clojure::rust::Object::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Comparator::init();
        clojure::rust::Serializable::init();
        clojure::lang::AFn::init();
        clojure::lang::IObj::init();
    }
}

pub trait AFunction: AFn + IObj + Comparator + Serializable {
    /// AFunction -> MethodImplCache
    fn __methodImplCache(&self) -> ObjResult<Object>;
}


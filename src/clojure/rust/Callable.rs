// Callable

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::ObjResult;
}

init_obj! {
    Callable {
        clojure::rust::Object::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait Callable {
    fn call(&self) -> ObjResult<Object>;
}
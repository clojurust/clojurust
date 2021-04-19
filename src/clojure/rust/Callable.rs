// Callable

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::ObjError;
}

init_obj! {
    Callable {
        clojure::rust::Object::init();
        clojure::rust::ObjError::init();
    }
}

pub trait Callable {
    fn call(&self) -> ObjResult<Object>;
}
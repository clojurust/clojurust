// Callable

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

init_obj! {
    Callable {
        clojure::rust::Object::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait Callable {
    fn call(&self) -> ObjResult<Object>;
}
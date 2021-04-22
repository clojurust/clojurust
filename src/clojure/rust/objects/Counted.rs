/// Protocol `Counted`

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

init_obj! {
    Counted {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait Counted: IObject {
    /// give the nr of elements
    fn count(&self) -> ObjResult<usize>;
}
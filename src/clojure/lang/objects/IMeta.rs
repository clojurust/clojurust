//! Protocol IMeta

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

init_obj! {
    IMeta {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait IMeta: IObject {
    fn meta(&self) -> ObjResult<Object>;
}

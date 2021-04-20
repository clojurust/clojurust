//! Protocol IMeta

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::ObjResult;
}

init_obj! {
    Runnable {
        clojure::rust::Object::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait IMeta: IObject {
    fn meta(&self) -> ObjResult<Object>;
}

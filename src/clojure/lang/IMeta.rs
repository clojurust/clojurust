//! Protocol IMeta

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
}

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

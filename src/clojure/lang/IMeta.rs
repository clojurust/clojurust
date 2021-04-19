//! Protocol IMeta

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::ObjError;
}

init_obj! {
    Runnable {
        clojure::rust::Object::init();
        clojure::rust::ObjError::init();
    }
}

pub trait IMeta: TObject {
    fn meta(&self) -> ObjResult<Object>;
}

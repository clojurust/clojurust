//! Protocol IHashEq

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
}

init_obj! {
    IHashEq {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait IHashEq: IObject {
    fn hasheq(&self) -> ObjResult<usize>;
}


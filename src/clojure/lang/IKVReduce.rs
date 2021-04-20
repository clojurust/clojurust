//! Protocol IKVReduce

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
}

init_obj! {
    IKVReduce {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait IKVReduce: IObject {
    fn kvreduce(&self, f: Object, init: Object) -> ObjResult<Object>;
}
//! Protocol IKVReduce

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

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
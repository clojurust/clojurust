//! Protocol IKVReduce

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjResult::*;

pub trait IKVReduce: IObject {
    fn kvreduce(&self, f: Object, init: Object) -> ObjResult<Object>;
}
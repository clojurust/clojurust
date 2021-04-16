//! Protocol IKVReduce

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;

pub trait IKVReduce: TObject {
    fn kvreduce(&self, f: Object, init: Object) -> ObjResult<Object>;
}
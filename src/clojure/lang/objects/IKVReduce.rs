//! Protocol IKVReduce

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait IKVReduce: IObject {
    fn kvreduce(&self, f: Object, init: Object) -> ObjResult<Object>;
}
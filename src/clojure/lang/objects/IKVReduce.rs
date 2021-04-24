//! Protocol IKVReduce

// use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait IKVReduce: IObject
{
    fn kvreduce(
        &self,
        f: Object,
        init: Object,
    ) -> ObjResult<Object>;
}

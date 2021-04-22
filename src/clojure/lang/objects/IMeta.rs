//! Protocol IMeta

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait IMeta: IObject {
    fn meta(&self) -> ObjResult<Object>;
}

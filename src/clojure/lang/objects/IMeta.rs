//! Protocol IMeta

use clojure::rust::*;

use crate::*;
// use clojure::lang::*;

pub trait IMeta: IObject
{
    fn meta(&self) -> ObjResult<Object>;
}

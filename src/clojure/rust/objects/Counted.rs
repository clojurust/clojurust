use clojure::rust::*;

/// Protocol `Counted`
use crate::*;
// use clojure::lang::*;

pub trait Counted: IObject
{
    /// give the nr of elements
    fn count(&self) -> ObjResult<usize>;
}

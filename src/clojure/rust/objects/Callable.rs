// Callable

use clojure::rust::*;

use crate::*;
// use clojure::lang::*;

pub trait Callable
{
    fn call(&self) -> ObjResult<Object>;
}

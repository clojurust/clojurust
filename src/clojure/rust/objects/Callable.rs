// Callable

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait Callable {
    fn call(&self) -> ObjResult<Object>;
}
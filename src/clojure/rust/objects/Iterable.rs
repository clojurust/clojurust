/// Iterable

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait Iterable: IObject {
    /// Iterable -> Iterator
    fn iterator(&self) -> ObjResult<Object>;
}
use clojure::rust::*;

/// Iterable
use crate::*;
// use clojure::lang::*;

pub trait Iterable: IObject {
    /// Iterable -> Iterator
    fn iterator(&self) -> ObjResult<Object>;
}

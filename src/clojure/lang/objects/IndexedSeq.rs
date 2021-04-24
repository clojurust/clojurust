use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait IndexedSeq:
    IObject+Counted+IPersistentCollection+ISeq+Sequable+Sequential
{
    fn index(&self) -> ObjResult<usize>;
}

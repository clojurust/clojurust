
use crate::*;
use clojure::rust::*;
use clojure::lang::*;

pub trait IndexedSeq: IObject + Counted + IPersistentCollection + ISeq 
        + Sequable + Sequential {
    fn index(&self) -> ObjResult<usize>;
}
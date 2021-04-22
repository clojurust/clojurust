//! Protocol ISeq

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

pub trait ISeq: IObject + Sequable + IPersistentCollection {
    /// ISeq -> Object -> ISeq
    fn cons(&self, o: &Object) -> ObjResult<Object>;
}
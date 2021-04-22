//! Protocol IPersistentCollection

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

pub trait IPersistentCollection: IObject + Sequable {
    fn cons(&self, o: &Object) -> ObjResult<Object>;
    fn count(&self) -> ObjResult<usize>;
    fn empty(&self) -> ObjResult<Object>;
    fn equiv(&self, o: Object) -> ObjResult<bool>;
}
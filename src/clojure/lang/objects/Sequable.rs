//! Protocol sequable

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait Sequable: IObject {
    /// Sequable -> ISeq
    fn seq(&self) -> ObjResult<Object>;
}
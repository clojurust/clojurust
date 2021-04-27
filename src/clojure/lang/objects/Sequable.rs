//! Protocol sequable

use clojure::rust::*;

use crate::*;
// use clojure::lang::*;

pub trait Sequable: IObject {
    /// Sequable -> ISeq
    fn seq(&self) -> ObjResult<Object>;
}

//! Protocol Reversible

use clojure::rust::*;

use crate::*;
// use clojure::lang::*;

pub trait Reversible: IObject {
    /// Reversible -> ISeq
    fn rseq(&self) -> ObjResult<Object>;
}

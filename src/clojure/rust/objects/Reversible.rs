//! Protocol Reversible

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait Reversible: IObject {
    /// Reversible -> ISeq
    fn rseq(&self) -> ObjResult<Object>;
}


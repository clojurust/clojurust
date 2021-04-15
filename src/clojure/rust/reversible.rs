//! Protocol Reversible

use crate::clojure;
use clojure::rust::object::*;
use clojure::rust::obj_error::*;
use clojure::lang::i_seq::*;

pub trait Reversible: TObject {
    fn rseq(&self) -> ObjResult<&'_ ISeq>;
}


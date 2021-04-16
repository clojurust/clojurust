//! Protocol Reversible

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;
use clojure::lang::ISeq::*;

pub trait Reversible: TObject {
    fn rseq(&self) -> ObjResult<&'_ ISeq>;
}


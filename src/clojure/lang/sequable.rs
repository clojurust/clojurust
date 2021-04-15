//! Protocol sequable

use crate::clojure;
use clojure::rust::object::*;
use clojure::rust::obj_error::*;
use clojure::lang::i_seq::*;

pub trait Sequable: TObject {
    fn seq(&self) -> ObjResult<&'_ ISeq>;
}
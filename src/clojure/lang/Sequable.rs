//! Protocol sequable

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;
use clojure::lang::ISeq::*;

pub trait Sequable: TObject {
    fn seq(&self) -> ObjResult<&'_ ISeq>;
}
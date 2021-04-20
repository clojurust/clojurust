//! Protocol sequable

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::IObject::*;
use clojure::rust::ObjResult::*;

pub trait Sequable: IObject {
    /// Sequable -> ISeq
    fn seq(&self) -> ObjResult<Object>;
}
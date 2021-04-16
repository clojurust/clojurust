//! Protocol IMeta

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;
use clojure::lang::IPersistentMap::*;

pub trait IMeta: TObject {
    fn meta(&self) -> ObjResult<&'_ IPersistentMap>;
}
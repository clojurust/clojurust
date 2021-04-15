//! Protocol IMeta

use crate::clojure;
use clojure::rust::object::*;
use clojure::rust::obj_error::*;
use clojure::lang::i_persistent_map::*;

pub trait IMeta: TObject {
    fn meta(&self) -> ObjResult<&'_ IPersistentMap>;
}
//! Protocol IHashEq

use crate::clojure;
use clojure::rust::object::*;
use clojure::rust::obj_error::*;

pub trait IHashEq: TObject {
    fn hasheq(&self) -> ObjResult<usize>;
}


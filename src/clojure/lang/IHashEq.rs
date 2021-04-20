//! Protocol IHashEq

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjResult::*;

pub trait IHashEq: IObject {
    fn hasheq(&self) -> ObjResult<usize>;
}


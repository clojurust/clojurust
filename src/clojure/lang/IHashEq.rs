//! Protocol IHashEq

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;

pub trait IHashEq: TObject {
    fn hasheq(&self) -> ObjResult<usize>;
}


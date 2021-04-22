//! Protocol IHashEq

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait IHashEq: IObject {
    fn hasheq(&self) -> ObjResult<usize>;
}


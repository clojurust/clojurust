//! Protocol IHashEq

// use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait IHashEq: IObject {
    fn hasheq(&self) -> ObjResult<usize>;
}

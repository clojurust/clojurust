//! Protocol IHashEq

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    IHashEq {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait IHashEq: IObject {
    fn hasheq(&self) -> ObjResult<usize>;
}


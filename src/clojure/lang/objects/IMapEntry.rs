//! Protocol IMapEntry

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    IMeta {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Map::init();
    }
}

pub trait IMapEntry: IObject + Entry {
    fn key(&self) -> ObjResult<Object>;
    fn val(&self) -> ObjResult<Object>;    
}
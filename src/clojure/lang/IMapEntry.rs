//! Protocol IMapEntry

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjResult::*;
use clojure::rust::Map::*;

pub trait IMapEntry: IObject + Entry {
    fn key(&self) -> ObjResult<Object>;
    fn val(&self) -> ObjResult<Object>;    
}
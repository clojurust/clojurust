//! Protocol IMapEntry

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;
use clojure::rust::Map::*;

pub trait IMapEntry: TObject + Entry {
    fn key(&self) -> ObjResult<Object>;
    fn val(&self) -> ObjResult<Object>;    
}
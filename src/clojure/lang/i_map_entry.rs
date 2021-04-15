//! Protocol IMapEntry

use crate::clojure;
use clojure::rust::object::*;
use clojure::rust::obj_error::*;
use clojure::rust::map::*;

pub trait IMapEntry: TObject + Entry {
    fn key(&self) -> ObjResult<Object>;
    fn val(&self) -> ObjResult<Object>;    
}
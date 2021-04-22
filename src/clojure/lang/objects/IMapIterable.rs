
use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait IMapIterable: IObject + Iterator {
    /// IMapIterable -> Iterator
    fn keyIterator(&self) -> ObjResult<Object>; 

    /// IMapIterable -> Iterator
    fn valIterator(&self) -> ObjResult<Object>; 
}
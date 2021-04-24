use clojure::rust::*;

use crate::*;
// use clojure::lang::*;

pub trait IMapIterable: IObject+Iterator
{
    type Item = Object;

    /// IMapIterable -> Iterator
    fn keyIterator(&self) -> ObjResult<Object>;

    /// IMapIterable -> Iterator
    fn valIterator(&self) -> ObjResult<Object>;
}

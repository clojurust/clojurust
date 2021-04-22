//Protocol IObj

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

pub trait IObj: IObject + IMeta {
    fn withMeta(&self, meta: &Object) -> ObjResult<Object>;
}
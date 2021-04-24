// Protocol IObj

use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait IObj: IObject+IMeta
{
    fn withMeta(
        &self,
        meta: &Object,
    ) -> ObjResult<Object>;
}

use clojure::rust::*;

use crate::*;
// use clojure::lang::*;

pub trait Set: IObject {
    fn size(&self) -> usize;

    fn isEmpty(&self) -> bool;

    fn containsKey(
        &self,
        key: Object,
    ) -> bool;

    fn containsValue(
        &self,
        key: Object,
    ) -> bool;

    fn get(
        &self,
        key: Object,
    ) -> ObjResult<Object>;
}

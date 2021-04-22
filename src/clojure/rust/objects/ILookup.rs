
use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait ILookup: IObject {
    // ILookup -> Object -> Object
    fn valAt1(&self, key: Object) -> ObjResult<Object>;

    // ILookup -> Object -> Object -> Object
    fn valAt(&self, key: Object, notFound: Object) -> ObjResult<Object>;
}

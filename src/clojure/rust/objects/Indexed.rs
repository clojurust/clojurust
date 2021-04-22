/// Protocol `Indexed`

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

init_obj! {
    Indexed {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Counted::init();
    }
}

pub trait Indexed: IObject + Counted {
    /// Indexed -> usize -> Object
    fn nth_1(&self, i: usize) -> ObjResult<Object>;

    /// Indexed -> usize -> Object -> Object
    fn nth_2(&self, i: usize, notFound: Object) -> ObjResult<Object>;
}


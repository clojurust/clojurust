/// Protocol `Indexed`


use crate::clojure;
use clojure::rust::Counted::*;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;

// castable_to!(SGlobals => [sync] TObject, Globals);

pub trait Indexed: TObject + Counted {
    fn nth_1(&self, i: usize) -> ObjResult<Object>;
    fn nth_2(&self, i: usize, notFound: Object) -> ObjResult<Object>;
}


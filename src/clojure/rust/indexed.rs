/// Protocol `Indexed`


use crate::clojure;
use clojure::rust::counted::*;
use clojure::rust::object::*;

// castable_to!(SGlobals => [sync] TObject, Globals);

pub trait Indexed: Counted {
    fn nth_1(&self, i: usize) -> Object;
    fn nth_2(&self, i: usize, notFound: Object) -> Object;
}


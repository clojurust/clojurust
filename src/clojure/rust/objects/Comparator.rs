use clojure::rust::*;

/// Comparator
use crate::*;
// use clojure::lang::*;

pub trait Comparator: IObject {
    /// AFunction -> Object -> Object -> int
    fn compare(
        o1: Object,
        o2: Object,
    ) -> i8;
}

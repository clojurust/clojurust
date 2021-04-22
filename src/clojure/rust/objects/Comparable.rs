//! Protocol Comparable

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait Comparable: IObject {
    /// Compare with object
    ///
    /// result < 0 iff self < o 
    /// result > 0 iff self > o 
    /// result 0 0 iff self = o
    /// throw NullPointerException iff o = nil
    /// throw ClassCastException iff o and self are not comparable
    #[allow(non_snake_case)]
    fn compareTo(&self, o: &Object) -> ObjResult<i8>;
}
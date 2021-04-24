use clojure::rust::*;

/// Protocol `Indexed`
use crate::*;
// use clojure::lang::*;

pub trait Indexed: IObject+Counted
{
    /// Indexed -> usize -> Object
    fn nth_1(
        &self,
        i: usize,
    ) -> ObjResult<Object>;

    /// Indexed -> usize -> Object -> Object
    fn nth_2(
        &self,
        i: usize,
        notFound: Object,
    ) -> ObjResult<Object>;
}

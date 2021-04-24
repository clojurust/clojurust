use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait AFunction: AFn+IObj+Comparator+Serializable
{
    /// AFunction -> MethodImplCache
    fn __methodImplCache(&self) -> ObjResult<Object>;
}

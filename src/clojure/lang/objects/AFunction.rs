// use intertrait::cast::*;
use crate::*;
use clojure::rust::*;
use clojure::lang::*;

pub trait AFunction: AFn + IObj + Comparator + Serializable {
    /// AFunction -> MethodImplCache
    fn __methodImplCache(&self) -> ObjResult<Object>;
}


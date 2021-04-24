use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait IFn: IObject+Callable
{
    fn invoke(
        &self,
        args: &[&Object],
    ) -> ObjResult<Object>;
    fn apply(
        &self,
        arglist: &ISeq,
    ) -> ObjResult<Object>;
}

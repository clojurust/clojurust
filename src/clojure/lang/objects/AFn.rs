//! Protocol AFn

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

pub trait AFn: IFn {
    fn invoke(&self, args: &[&Object]) -> ObjResult<Object>; 
}


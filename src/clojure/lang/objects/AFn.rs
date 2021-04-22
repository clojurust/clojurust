//! Protocol AFn

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    AFn {
        clojure::rust::Object::init();
        clojure::rust::ObjResult::init();
        clojure::lang::IFn::init();
    }
}

pub trait AFn: IFn {
    fn invoke(&self, args: &[&Object]) -> ObjResult<Object>; 
}


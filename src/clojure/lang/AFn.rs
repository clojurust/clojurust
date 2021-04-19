//! Protocol AFn

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::ObjError;
    clojure::lang::IFn;
}

init_obj! {
    AFn {
        clojure::rust::Object::init();
        clojure::rust::ObjError::init();
        clojure::lang::IFn::init();
    }
}

pub trait AFn: IFn {
    fn invoke(&self, args: &[&Object]) -> ObjResult<Object>; 
}


//! Protocol AFn

use crate::*;

use_obj! {
    //    clojure::rust::Object;
    //    clojure::rust::Class;
    clojure::rust::ObjError;
    clojure::lang::IFn;
}

init_obj! {
    AFn {
        clojure::rust::IFn::init();
    }
}

pub trait AFn: IFn {
    fn invoke(&self, args: &[&Object]) -> ObjResult<Object>; 
}


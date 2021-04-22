
use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

init_obj! {
    Sequential {
        clojure::rust::IObject::init();
    }
}

pub trait MapEquivalence: IObject {
    
}
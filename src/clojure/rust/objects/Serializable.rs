
use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

init_obj! {
    Serializable {
        clojure::rust::IObject::init();
    }
}

pub trait Serializable: IObject {}
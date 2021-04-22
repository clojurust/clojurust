//! An object that maps keys to values. A map cannot contain duplicate keys; each key can map to at most one value.

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

init_obj! {
    Map {
        clojure::rust::IObject::init();
    }
}

pub trait Map: IObject {

}

pub trait Entry: IObject {

}


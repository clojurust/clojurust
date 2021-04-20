//! An object that maps keys to values. A map cannot contain duplicate keys; each key can map to at most one value.

use crate::*;

use_obj! {
    clojure::rust::IObject;
}

init_obj! {
    Map {
        clojure::rust::IObject::init();
    }
}

pub trait Map: IObject {

}

/// This is an entry for K = V = Object
pub trait MapEntry: IObject {

}



/// This is an entry for K = V = Object
use crate::*;

use_obj! {
    clojure::rust::IObject;
    clojure::lang::AMapEntry;
}

init_obj! {
    Map {
        clojure::rust::IObject::init();
        clojure::lang::AMapEntry::init();
    }
}

pub trait MapEntry: IObject + AMapEntry {

}
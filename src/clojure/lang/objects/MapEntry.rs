/// This is an entry for K = V = Object
use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    Map {
        clojure::rust::IObject::init();
        clojure::lang::AMapEntry::init();
    }
}

pub trait MapEntry: IObject + AMapEntry {

}
//! # HashMap of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` HashMap<Object,Object> library

use crate::*;
use clojure::rust::Map;

use_obj! {
    clojure::lang::IPersistentMap;
    clojure::lang::MapEquivalence;
    clojure::lang::IHashEq;
    clojure::rust::Serializable;
    clojure::rust::Iterable;
}

init_obj! {
    APersistentMap {
        clojure::lang::IPersistentMap::init();
        clojure::lang::MapEquivalence::init();
        clojure::lang::IHashEq::init();
        clojure::rust::Map::init();
        clojure::rust::Serializable::init();
        clojure::rust::Iterable::init();
    }
}

pub trait APersistentMap: IPersistentMap + Map::Map + Iterable + Serializable
        + MapEquivalence + IHashEq {

}

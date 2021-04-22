//! # HashMap of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` HashMap<Object,Object> library

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

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

pub trait APersistentMap: IPersistentMap + Map + Iterable + Serializable
        + MapEquivalence + IHashEq {

}

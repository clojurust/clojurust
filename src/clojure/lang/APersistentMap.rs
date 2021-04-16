//! # HashMap of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` HashMap<Object,Object> library

use crate::clojure;
use clojure::lang::IPersistentMap::*;
use clojure::lang::MapEquivalence::*;
use clojure::lang::IHashEq::*;
use clojure::rust::Map;
use clojure::rust::Serializable::*;
use clojure::rust::Iterable::*;

pub trait APersistentMap: IPersistentMap + Map::Map + Iterable + Serializable
        + MapEquivalence + IHashEq {

}

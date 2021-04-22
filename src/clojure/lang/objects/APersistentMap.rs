//! # HashMap of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` HashMap<Object,Object> library

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

pub trait APersistentMap: IPersistentMap + Map + Iterable + Serializable
        + MapEquivalence + IHashEq {

}

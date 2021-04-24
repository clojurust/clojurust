//! # HashMap of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` HashMap<Object,Object> library

use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait APersistentMap:
    IPersistentMap+Map+Iterable+Serializable+MapEquivalence+IHashEq
{
}

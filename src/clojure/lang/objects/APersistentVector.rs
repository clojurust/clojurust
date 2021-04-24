//! # Vector of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` Vector<Object> library

use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait APersistentVector: IObject+IPersistentVector+Iterable+List+RandomAccess+Comparable+Serializable+IHashEq
{
    fn _hash(&self) -> usize;
    fn _hash_eq(&self) -> usize;
}

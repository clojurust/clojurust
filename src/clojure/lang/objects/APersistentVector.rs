//! # Vector of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` Vector<Object> library

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

pub trait APersistentVector: IObject + IPersistentVector + Iterable 
                        + List + RandomAccess + Comparable + Serializable
                        + IHashEq 
{
    fn _hash(&self) -> usize;
    fn _hash_eq(&self) -> usize;
}



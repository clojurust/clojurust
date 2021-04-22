//! # HashSet of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` HashSet<Object> library

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

pub trait APersistentSet: IObject + AFn + IPersistentSet 
                + Collection + Set + Serializable + IHashEq {
    
}

//! # HashSet of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` HashSet<Object> library

use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait APersistentSet:
    IObject+AFn+IPersistentSet+Collection+Set+Serializable+IHashEq
{
}

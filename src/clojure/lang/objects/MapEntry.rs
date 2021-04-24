use clojure::lang::*;
use clojure::rust::*;

/// This is an entry for K = V = Object
use crate::*;

////////////////
/// Here me must implement im::hashmap::Entry
pub trait MapEntry: IObject+AMapEntry
{
}

/// This is an entry for K = V = Object
use crate::*;
use clojure::rust::*;
use clojure::lang::*;

////////////////
/// Here me must implement im::hashmap::Entry

pub trait MapEntry: IObject + AMapEntry {

}
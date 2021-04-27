//! Protocol IMapEntry

use clojure::rust::*;

use crate::*;
// use clojure::lang::*;

pub trait IMapEntry: IObject+Entry {}

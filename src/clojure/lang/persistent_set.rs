//! # HashSet of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` HashSet<Object> library

// use lazy_static::__Deref;
// use std::{fmt::*, hash::*};

use std::{fmt::*, sync::*};

use im::hashset;

// use intertrait::cast::*;

use crate::use_obj;

use_obj! {
    clojure::rust::object;
    clojure::rust::class;
}

castable_to!(SAPersistentSet => [sync] TObject, PersistentSet);

init_obj! {
    ObjHashSet {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

#[derive(Debug)]
pub struct SAPersistentSet {
    inner: hashset::HashSet<Object>
}

impl TObject for SAPersistentSet {
    fn get_class<'a>(&self) -> &'a SClass {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }
}

impl Default for SAPersistentSet {
    fn default() -> Self {
        SAPersistentSet {
            inner: hashset::HashSet::<Object>::default()
        }
    
    }
}

impl Display for SAPersistentSet {
    /// Return string representation of
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "SObjHashSet {:?}", self.inner)
    }
}

pub trait PersistentSet: CastFromSync {
    fn new() -> Object
    where
        Self: Sized;
}

use crate::new_obj;

impl PersistentSet for SAPersistentSet {
    fn new() -> Object {
        new_obj!(SAPersistentSet::default())
    }
}

impl PersistentSet {}

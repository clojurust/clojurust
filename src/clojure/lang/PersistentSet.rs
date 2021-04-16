//! # HashSet of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` HashSet<Object> library

// use lazy_static::__Deref;
// use std::{fmt::*, hash::*};

use std::{fmt::*};

use im::hashset;

// use intertrait::cast::*;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::Class;
    clojure::lang::APersistentSet;
}

castable_to!(SPersistentSet => [sync] TObject, PersistentSet);

init_obj! {
    ObjHashSet {
        clojure::rust::Object::init();
        clojure::rust::Class::init();
        // clojure::lang::a_persistent_set::init();
    }
}

#[derive(Debug)]
pub struct SPersistentSet {
    inner: hashset::HashSet<Object>
}

impl TObject for SPersistentSet {
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

impl Default for SPersistentSet {
    fn default() -> Self {
        SPersistentSet {
            inner: hashset::HashSet::<Object>::default()
        }
    
    }
}

impl Display for SPersistentSet {
    /// Return string representation of
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "PersistentSet {:?}", self.inner)
    }
}

pub trait PersistentSet: TObject + APersistentSet {
}

impl PersistentSet for SPersistentSet {
}

impl APersistentSet for SPersistentSet {
}

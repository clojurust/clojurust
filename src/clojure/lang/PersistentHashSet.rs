//! # HashSet of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` HashSet<Object> library

// use lazy_static::__Deref;
// use std::{fmt::*, hash::*};

use std::{fmt::*};

use im::hashset::*;
use im::hashmap::*;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::lang::IObj;
    clojure::rust::Class;
    clojure::lang::APersistentSet;
}

castable_to!(SPersistentHashSet => 
        [sync] IObject, PersistentHashSet, IObj, APersistentSet);

init_obj! {
    PersistentHashSet {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::lang::IObj::init();
        clojure::rust::Class::init();
        // clojure::lang::a_persistent_set::init();
    }
}

#[derive(Debug)]
pub struct SPersistentHashSet {
    /// hashmap::HashMap<Object>
    meta: Object,
    /// hashset::HashSet<Object>
    inner: Object 
}

impl IObject for SPersistentHashSet {
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

impl Default for SPersistentHashSet {
    fn default() -> Self {
        SPersistentHashSet {
            meta: HashMap::<Object>::default(),
            inner: HashSet::<Object>::default()
        }
    
    }
}

impl Display for SPersistentHashSet {
    /// Return string representation of
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "PersistentSet {:?}", self.inner)
    }
}

pub trait PersistentHashSet: IObject + IObj + APersistentSet {
}

impl PersistentHashSet for SPersistentHashSet {
}

impl APersistentSet for SPersistentHashSet {
}

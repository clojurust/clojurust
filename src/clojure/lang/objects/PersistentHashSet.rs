//! # HashSet of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` HashSet<Object> library

// use lazy_static::__Deref;
// use std::{fmt::*, hash::*};
use std::sync::*;

use im::hashset::*;
use im::hashmap::*;

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

use intertrait::*;
castable_to!(SPersistentHashSet => 
        [sync] IObject, PersistentHashSet, IObj, IMeta, APersistentSet);

init_obj! {
    PersistentHashSet {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::lang::IObj::init();
        clojure::rust::Class::init();
        clojure::lang::APersistentSet::init();
        clojure::lang::IMeta::init();
        }
}

pub struct SPersistentHashSet {
    /// hashmap::HashMap<Object>
    meta: Arc<HashMap<Object, Object>>,
    /// hashset::HashSet<Object>
    inner: Arc<HashSet<Object>>
}

impl IObject for SPersistentHashSet {
    #[allow(non_snake_case)]
    fn getClass<'a>(&self) -> &'a SClass {
        todo!()
    }

    #[allow(non_snake_case)]
    fn hashCode(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }

    #[allow(non_snake_case)]
    fn toString(&self) -> usize {
        todo!()
    }
}

impl Default for SPersistentHashSet {
    fn default() -> Self {
        SPersistentHashSet {
            meta: Arc::new(HashMap::<Object, Object>::default()),
            inner: Arc::new(HashSet::<Object>::default())
        }
    
    }
}

pub trait PersistentHashSet: IObject + IObj + APersistentSet {
}

impl PersistentHashSet for SPersistentHashSet {
}

impl IMeta for SPersistentHashSet {
    fn meta(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl IObj for SPersistentHashSet {
    fn withMeta(&self, meta: &Object) -> clojure::rust::ObjResult::ObjResult<Object> {
        todo!()
    }
}

impl APersistentSet for SPersistentHashSet {
}

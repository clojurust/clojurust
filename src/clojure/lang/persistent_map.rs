//! # HashMap of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` HashMap<Object,Object> library

// use intertrait::cast::*;

use std::fmt::*;
use im::hashmap;

use crate::use_obj;

use_obj! {
    clojure::rust::object;
    clojure::rust::class;
}

castable_to!(SPersistentMap => [sync] TObject, PersistentMap);

init_obj! {
    SPersistentMap {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

#[derive(Debug)]
pub struct SPersistentMap {
    inner: hashmap::HashMap<Object, Object>
}

impl TObject for SPersistentMap {
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

impl Default for SPersistentMap {
    fn default() -> Self {
        SPersistentMap {
            inner: hashmap::HashMap::<Object, Object>::default()
        }
    
    }
}

impl Display for SPersistentMap {
    /// Return string representation of
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "SPersistentMap {:?}", self.inner)
    }
}

pub trait PersistentMap: TObject {
}

impl PersistentMap for SPersistentMap {
}

impl PersistentMap {}

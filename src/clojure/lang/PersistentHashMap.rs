//! # HashMap of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` HashMap<Object,Object> library

// use intertrait::cast::*;

use std::fmt::*;
use im::hashmap;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::ObjError;
    clojure::rust::Class;
    clojure::lang::IPersistentMap;
    clojure::rust::Counted;
    clojure::rust::Iterable;
    clojure::lang::IMeta;
    clojure::rust::Associative;
}

castable_to!(SPersistentHashMap => [sync] TObject, PersistentHashMap, Counted, 
                                Iterable, IMeta, Associative);

init_obj! {
    SPersistentHashMap {
        clojure::rust::Object::init();
        clojure::rust::Class::init();
    }
}

#[derive(Debug)]
pub struct SPersistentHashMap {
    inner: hashmap::HashMap<Object, Object>
}

pub trait PersistentHashMap: TObject {
}

impl IPersistentMap for SPersistentHashMap {
    fn assoc(&self, key: Object, val: Object) -> ObjResult<&'_ IPersistentMap> {
        todo!()
    }

    #[allow(non_snake_case)]
    fn assocEx(&self, key: Object, val: Object) -> ObjResult<&'_ IPersistentMap> {
        todo!()
    }

    fn without(&self, key: Object) -> ObjResult<&'_ IPersistentMap> {
        todo!()
    }
}

impl Counted for SPersistentHashMap {
    fn count(&self) -> ObjResult<usize> {
        todo!()
    }
}
    
impl IMeta for SPersistentHashMap {
    fn meta(&self) -> ObjResult<Object> {
        todo!()
    }
}
    
impl Iterable for SPersistentHashMap {
}
    
impl Associative for SPersistentHashMap {
    fn assoc(&self, key: &Object, value: &Object) -> ObjResult<&Associative> {
        todo!()
    }

    #[allow(non_snake_case)]
    fn containsKey(&self, key: &Object) -> ObjResult<bool> {
        todo!()
    }

    #[allow(non_snake_case)]
    fn entryAt(&self, key: &Object) -> ObjResult<&super::IMapEntry::IMapEntry> {
        todo!()
    }
}
    
impl TObject for SPersistentHashMap {
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

impl Default for SPersistentHashMap {
    fn default() -> Self {
        SPersistentHashMap {
            inner: hashmap::HashMap::<Object, Object>::default()
        }
    
    }
}

impl Display for SPersistentHashMap {
    /// Return string representation of
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "SPersistentMap {:?}", self.inner)
    }
}

impl PersistentHashMap for SPersistentHashMap {
}

impl PersistentHashMap {}

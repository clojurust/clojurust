//! # HashMap of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` HashMap<Object,Object> library

// use intertrait::cast::*;

use std::fmt::*;
use im::hashmap;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
    clojure::rust::Class;
    clojure::lang::IPersistentMap;
    clojure::rust::Counted;
    clojure::rust::Iterable;
    clojure::lang::IMeta;
    clojure::rust::Associative;
}

castable_to!(SPersistentHashMap => [sync] IObject, PersistentHashMap, Counted, 
                                Iterable, IMeta, Associative);

init_obj! {
    PersistentHashMap {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Class::init();
        clojure::lang::IPersistentMap::init();
        clojure::rust::Counted::init();
        clojure::rust::Iterable::init();
        clojure::lang::IMeta::init();
        clojure::rust::Associative::init();
        }
}

#[derive(Debug)]
pub struct SPersistentHashMap {
    inner: hashmap::HashMap<Object, Object>
}

pub trait PersistentHashMap: IObject {
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
    
impl IObject for SPersistentHashMap {
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

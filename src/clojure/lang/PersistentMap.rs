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
    clojure::rust::Associative;
}

castable_to!(SPersistentMap => [sync] TObject, PersistentMap, Counted, 
                                Iterable, Associative);

init_obj! {
    SPersistentMap {
        clojure::rust::Object::init();
        clojure::rust::Class::init();
    }
}

#[derive(Debug)]
pub struct SPersistentMap {
    inner: hashmap::HashMap<Object, Object>
}

impl IPersistentMap for SPersistentMap {
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

impl Counted for SPersistentMap {
    fn count(&self) -> ObjResult<usize> {
        todo!()
    }
}
    
impl Iterable for SPersistentMap {
}
    
impl Associative for SPersistentMap {
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

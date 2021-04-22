//! # HashMap of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` HashMap<Object,Object> library

// use intertrait::cast::*;

use im::hashmap::*;

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

use intertrait::*;
castable_to!(SPersistentHashMap => [sync] IObject, PersistentHashMap, Counted, 
                                Iterable, IObj, IMeta, Associative);

init_obj! {
    PersistentHashMap {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Class::init();
        clojure::lang::IPersistentMap::init();
        clojure::rust::Counted::init();
        clojure::rust::Iterable::init();
        clojure::lang::IObj::init();
        clojure::lang::IMeta::init();
        clojure::rust::Associative::init();
        clojure::lang::IEditableCollection::init();
        clojure::lang::IMapIterable::init();
        clojure::lang::IKVReduce::init();
    }
}

pub struct SPersistentHashMap {
    inner: HashMap<Object, Object>
}

pub trait PersistentHashMap: IObject + IEditableCollection + IObj +
                IMapIterable + IKVReduce {
}

impl IPersistentMap for SPersistentHashMap {
    fn assoc(&self, key: Object, val: Object) -> ObjResult<Object> {
        todo!()
    }

    #[allow(non_snake_case)]
    fn assocEx(&self, key: Object, val: Object) -> ObjResult<Object> {
        todo!()
    }

    fn without(&self, key: Object) -> ObjResult<Object> {
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
    fn iterator(&self) -> ObjResult<Object> {
        todo!()
    }
}
    
impl Associative for SPersistentHashMap {
    fn assoc(&self, key: &Object, value: &Object) -> ObjResult<Object> {
        todo!()
    }

    #[allow(non_snake_case)]
    fn containsKey(&self, key: &Object) -> ObjResult<bool> {
        todo!()
    }

    #[allow(non_snake_case)]
    fn entryAt(&self, key: &Object) -> ObjResult<Object> {
        todo!()
    }
}
    
impl IObject for SPersistentHashMap {
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

impl Default for SPersistentHashMap {
    fn default() -> Self {
        SPersistentHashMap {
            inner: HashMap::<Object, Object>::default()
        }
    
    }
}

impl PersistentHashMap for SPersistentHashMap {
}

impl IMapIterable for SPersistentHashMap {
    fn keyIterator(&self) -> ObjResult<Object> {
        todo!()
    }

    fn valIterator(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl IObj for SPersistentHashMap {
    fn withMeta(&self, meta: &Object) -> ObjResult<Object> {
        todo!()
    }
}

impl IEditableCollection for SPersistentHashMap {
    fn asTransient(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl IKVReduce for SPersistentHashMap {
    fn kvreduce(&self, f: Object, init: Object) -> ObjResult<Object> {
        todo!()
    }
}


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
    fn toString(&self) -> String {
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
    fn withMeta(&self, meta: &Object) -> ObjResult<Object> {
        todo!()
    }
}

impl APersistentSet for SPersistentHashSet {
}

impl IHashEq for SPersistentHashSet {
    fn hasheq(&self) -> ObjResult<usize> {
        todo!()
    }
}

impl Serializable for SPersistentHashSet {
}

impl Set for SPersistentHashSet {
    fn size(&self) -> usize {
        todo!()
    }

    fn isEmpty(&self) -> bool {
        todo!()
    }

    fn containsKey(&self, key: Object) -> bool {
        todo!()
    }

    fn containsValue(&self, key: Object) -> bool {
        todo!()
    }

    fn get(&self, key: Object) -> ObjResult<Object> {
        todo!()
    }
}

impl IPersistentSet for SPersistentHashSet {
    fn disjoin(&self, key: Object) -> ObjResult<Object> {
        todo!()
    }

    fn assocEx(&self, key: Object) -> ObjResult<bool> {
        todo!()
    }

    fn get(&self, key: Object) -> ObjResult<Object> {
        todo!()
    }
}

impl Iterable for SPersistentHashSet {
    fn iterator(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl IPersistentCollection for SPersistentHashSet {
    fn cons(&self, o: &Object) -> ObjResult<Object> {
        todo!()
    }

    fn count(&self) -> ObjResult<usize> {
        todo!()
    }

    fn empty(&self) -> ObjResult<Object> {
        todo!()
    }

    fn equiv(&self, o: Object) -> ObjResult<bool> {
        todo!()
    }
}

impl Counted for SPersistentHashSet {
    fn count(&self) -> ObjResult<usize> {
        todo!()
    }
}

impl Sequable for SPersistentHashSet {
    fn seq(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl AFn for SPersistentHashSet {
    fn invoke(&self, args: &[&Object]) -> ObjResult<Object> {
        todo!()
    }
}

impl IFn for SPersistentHashSet {
}

impl Callable for SPersistentHashSet {
    fn call(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl Collection for SPersistentHashSet {
    fn size(&self) -> usize {
        todo!()
    }

    fn isEmpty(&self) -> bool {
        todo!()
    }

    fn contains(&self) -> ObjResult<bool> {
        todo!()
    }

    fn toArray(&self) -> ObjResult<Vec<Object>> {
        todo!()
    }

    fn containsAll(&self, c: &Object) -> ObjResult<bool> {
        todo!()
    }
}


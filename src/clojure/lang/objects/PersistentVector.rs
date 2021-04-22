//! # Vector of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` Vector<Object> library

// use lazy_static::__Deref;
// use std::{fmt::*, hash::*};
use std::sync::*;

// use im::hashmap::*;
use im::vector::*;

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

use intertrait::*;
castable_to!(SPersistentVector => [sync] IObject, PersistentVector, APersistentVector, 
                                        IObj, Counted, Indexed, IEditableCollection, IKVReduce);

init_obj! {
    PersistentVector {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::Class::init();
        clojure::rust::Serializable::init();
        clojure::rust::Comparable::init();
        clojure::rust::Reversible::init();
        clojure::rust::Counted::init();
        clojure::rust::Iterable::init();
        clojure::rust::Indexed::init();
        clojure::rust::Associative::init();
        clojure::rust::ObjResult::init();
        clojure::rust::RandomAccess::init();
        clojure::rust::List::init();
        clojure::lang::APersistentVector::init();
        clojure::lang::IPersistentVector::init();
        clojure::lang::IPersistentStack::init();
        clojure::lang::IPersistentCollection::init();
        clojure::lang::ITransientCollection::init();
        clojure::lang::IObj::init();
        clojure::lang::IMeta::init();
        clojure::lang::IHashEq::init();
        clojure::lang::Sequable::init();
        clojure::lang::IKVReduce::init();
        clojure::lang::IEditableCollection::init();
        clojure::rust::Collection::init();
    }
}

/////////////////////////////////
// Objects
pub struct SPersistentVector {
    _hash: usize,
    _hash_eq: usize, 
    /// hashmap::HashMap<Object>
    meta: Object,
    ///
    inner: Object 
}

//////////////////////////////////
// Protocols
pub trait PersistentVector: IObject + APersistentVector 
                    + IObj + IEditableCollection + IKVReduce {
}

//////////////////////////////////
// Implementations
impl PersistentVector for SPersistentVector {
}

impl Collection for SPersistentVector {
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

impl APersistentVector for SPersistentVector {
    fn _hash(&self) -> usize {
        self._hash
    }

    fn _hash_eq(&self) -> usize {
        self._hash_eq
    }
}

impl IKVReduce for SPersistentVector {
    fn kvreduce(&self, f: Object, init: Object) -> ObjResult<Object> {
        todo!()
    }
}

impl IHashEq for SPersistentVector {
    fn hasheq(&self) -> ObjResult<usize> {
        todo!()
    }
}

impl Serializable for SPersistentVector {
}

impl Comparable for SPersistentVector {
    #[allow(non_snake_case)]
    fn compareTo(&self, o: &Object) -> ObjResult<i8> {
        todo!()
    }
}

impl RandomAccess for SPersistentVector {
}

impl IEditableCollection for SPersistentVector {
    #[allow(non_snake_case)]
    fn asTransient(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl IObj for SPersistentVector {
    #[allow(non_snake_case)]
    fn withMeta(&self, meta: &Object) -> ObjResult<Object> {
        todo!()
    }
}

impl IMeta for SPersistentVector {
    fn meta(&self) -> ObjResult<Object> {
        Ok(self.meta)
    }
}

impl List for SPersistentVector {
}

impl IPersistentVector for SPersistentVector {
    #[allow(non_snake_case)]
    fn assocN(&self, i: usize, val: &Object) -> ObjResult<Object> {
        todo!()
    }

    fn cons(&self, o: Object) -> ObjResult<Object> {
        todo!()
    }

    fn length(&self) -> ObjResult<usize> {
        todo!()
    }
}

impl IPersistentStack for SPersistentVector {
    fn peek(&self) -> ObjResult<Object> {
        todo!()
    }

    fn pop(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl IPersistentCollection for SPersistentVector {
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

impl Indexed for SPersistentVector {
    fn nth_1(&self, i: usize) -> ObjResult<Object> {
        todo!()
    }

    fn nth_2(&self, i: usize, notFound: Object) -> ObjResult<Object> {
        todo!()
    }
}

impl Counted for SPersistentVector {
    fn count(&self) -> ObjResult<usize> {
        todo!()
    }
}

impl Reversible for SPersistentVector {
    fn rseq(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl Sequable for SPersistentVector {
    fn seq(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl Associative for SPersistentVector {
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

impl Iterable for SPersistentVector {
    fn iterator(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl IObject for SPersistentVector {
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

impl Default for SPersistentVector {
    fn default() -> Self {
        SPersistentVector {
            _hash: 0,
            _hash_eq: 0,
            meta: Object::null(),
            inner: new_obj!(Vector::<Object>::default())
        }
    }
}

impl IObject for Vector::<Object> {
    fn getClass<'a>(&self) -> &'a SClass {
        todo!()
    }

    fn hashCode(&self) -> usize {
        todo!()
    }

    fn toString(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }
}


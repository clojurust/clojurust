//! # Vector of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` Vector<Object> library

// use lazy_static::__Deref;
// use std::{fmt::*, hash::*};
use std::{fmt::*};

use im::vector;

// use intertrait::cast::*;
use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::Class;
    clojure::rust::Serializable;
    clojure::rust::Comparable;
    clojure::rust::Reversible;
    clojure::rust::Counted;
    clojure::rust::Iterable;
    clojure::rust::Indexed;
    clojure::rust::Associative;
    clojure::rust::ObjError;
    clojure::rust::RandomAccess;
    clojure::rust::List;
    clojure::lang::APersistentVector;
    clojure::lang::IPersistentVector;
    clojure::lang::IPersistentStack;
    clojure::lang::IPersistentCollection;
    clojure::lang::ITransientCollection;
    clojure::lang::IObj;
    clojure::lang::IMeta;
    clojure::lang::IHashEq;
    clojure::lang::Sequable;
    clojure::lang::IKVReduce;
    clojure::lang::IEditableCollection;
}

castable_to!(SPersistentVector => [sync] TObject, PersistentVector, APersistentVector, 
                                        IObj, Counted, Indexed, IEditableCollection, IKVReduce);

init_obj! {
    PersistentVector {
        clojure::rust::Object::init();
        clojure::rust::Class::init();
    }
}

/////////////////////////////////
// Objects
#[derive(Debug)]
pub struct SPersistentVector {
    meta: Object,
    inner: im::vector::Vector<Object>
}

//////////////////////////////////
// Protocols
pub trait PersistentVector: TObject + APersistentVector 
                    + IObj + IEditableCollection + IKVReduce {
}

//////////////////////////////////
// Implementations
impl PersistentVector for SPersistentVector {
}

impl APersistentVector for SPersistentVector {
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
    fn asTransient(&self) -> ObjResult<&'_ super::ITransientCollection::ITransientCollection> {
        todo!()
    }
}

impl IObj for SPersistentVector {
    #[allow(non_snake_case)]
    fn withMeta(&self, meta: &Object) -> ObjResult<&'_ IObj> {
        todo!()
    }
}

impl IMeta for SPersistentVector {
    fn meta(&self) -> ObjResult<&'_ super::IPersistentMap::IPersistentMap> {
        todo!()
    }
}

impl List for SPersistentVector {
}

impl IPersistentVector for SPersistentVector {
    #[allow(non_snake_case)]
    fn assocN(&self, i: usize, val: &Object) -> ObjResult<&'_ IPersistentVector> {
        todo!()
    }

    fn cons(&self, o: Object) -> ObjResult<&'_ IPersistentVector> {
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

    fn pop(&self) -> ObjResult<&'_ IPersistentStack> {
        todo!()
    }
}

impl IPersistentCollection for SPersistentVector {
    fn cons(&self, o: &Object) -> ObjResult<&'_ IPersistentCollection> {
        todo!()
    }

    fn count(&self) -> ObjResult<usize> {
        todo!()
    }

    fn empty(&self) -> ObjResult<&'_ IPersistentCollection> {
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
    fn rseq(&self) -> ObjResult<&'_ super::ISeq::ISeq> {
        todo!()
    }
}

impl Sequable for SPersistentVector {
    fn seq(&self) -> ObjResult<&'_ super::ISeq::ISeq> {
        todo!()
    }
}

impl Associative for SPersistentVector {
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

impl Iterable for SPersistentVector {
}

impl TObject for SPersistentVector {
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

impl Default for SPersistentVector {
    fn default() -> Self {
        SPersistentVector {
            meta: Object::null(),
            inner: vector::Vector::<Object>::default()
        }
    }
}

impl Display for SPersistentVector {
    /// Return string representation of vector
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "^{:?} {:?}", self.meta(), self.inner)
    }
}


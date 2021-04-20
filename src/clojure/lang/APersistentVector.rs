//! # Vector of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` Vector<Object> library

use std::fmt::*;

use crate::*;

use_obj! {
    // APersistentVector
    clojure::rust::Object;
    // clojure::rust::ObjError
    clojure::rust::List;
    clojure::rust::RandomAccess;
    clojure::rust::Comparable;
    clojure::rust::Iterable;
    clojure::rust::Reversible;
    clojure::rust::Indexed;
    clojure::rust::Serializable;
    clojure::lang::IPersistentCollection;
    clojure::lang::IPersistentStack;
    clojure::lang::IPersistentVector;
    clojure::lang::IHashEq;
    clojure::lang::Sequable;

    // RSeq
    clojure::lang::IndexedSeq;
    clojure::rust::Counted;
    clojure::lang::ASeq;

    // SubVector
    clojure::lang::IObj;
    clojure::lang::IMeta;
}

init_obj! {
    APersistentVector {
        // APersistentVector
        clojure::rust::Object::init();
        clojure::lang::IPersistentVector::init();
        clojure::rust::List::init();
        clojure::rust::RandomAccess::init();
        clojure::rust::Comparable::init();
        clojure::rust::Iterable::init();
        clojure::rust::Serializable::init();
        clojure::lang::IHashEq::init();

        // RSeq
        clojure::lang::IndexedSeq::init();
        clojure::rust::Counted::init();
        clojure::lang::ASeq::init();

        // SubVector
        clojure::lang::IObj::init();
        clojure::lang::IMeta::init();
    }
}

// castable_to!(SAPersistentVector => [sync] APersistentVector, IObject, IPersistentVector,  
//     List, RandomAccess, Comparable, Serializable, IHashEq);

// #[derive(Debug)]
// pub struct SAPersistentVector {
//     _hash: usize,
//     _hash_eq: usize,
// }

pub trait APersistentVector: IObject + IPersistentVector + Iterable 
                        + List + RandomAccess + Comparable + Serializable
                        + IHashEq 
{
    fn _hash(&self) -> usize;
    fn _hash_eq(&self) -> usize;
}

pub struct SRSeq {
    v: Object, //&'a IPersistantVector,
    i: usize,
}

pub trait RSeq: IObject + ASeq + IndexedSeq + Counted {

}

#[derive(Debug)]
pub struct SSubVector {
    v: Object, // &'a IPersistantVector,
    start: usize,
    end: usize,
    _meta: Object, //&'a IPersistantMap,
}

pub trait SubVector: IObject + IObj {
    fn v(&self) -> Object;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
}

impl SubVector for SSubVector {
    fn v(&self) -> Object {
        self.v.clone()
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }
}

impl IObj for SSubVector {
    fn withMeta(&self, meta: &Object) -> clojure::rust::ObjResult::ObjResult<&'_ IObj> {
        todo!()
    }
}

impl IMeta for SSubVector {
    fn meta(&self) -> clojure::rust::ObjResult::ObjResult<&'_ super::IPersistentMap::IPersistentMap> {
        Ok(self._meta.clone())
    }
}

impl IObject for SSubVector {
    fn get_class<'a>(&self) -> &'a clojure::rust::Class::SClass {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }
}

impl Display for SSubVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        todo!()
    }
}


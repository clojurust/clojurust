//! # Vector of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` Vector<Object> library

// APersistentVector
use crate::clojure;
use clojure::rust::Object::*;
use clojure::lang::IPersistentVector::*;
use clojure::rust::List::*;
use clojure::rust::RandomAccess::*;
use clojure::rust::Comparable::*;
use clojure::rust::Iterable::*;
use clojure::rust::Serializable::*;
use clojure::lang::IHashEq::*;

// RSeq
use clojure::lang::IndexedSeq::*;
use clojure::rust::Counted::*;
use clojure::lang::ASeq::*;

// SubVector
use clojure::lang::IObj::*;

// Return values
// use clojure::rust::ObjError::*;

pub struct SAPersistantVector {
    _hash: usize,
    _hash_eq: usize,
}

pub trait APersistentVector: TObject + IPersistentVector + Iterable 
                        + List + RandomAccess + Comparable + Serializable
                        + IHashEq {

}

pub struct SRSeq {
    v: Object, //&'a IPersistantVector,
    i: usize,
}

pub trait RSeq: TObject + ASeq + IndexedSeq + Counted {

}

pub struct SSubVector {
    v: Object, // &'a IPersistantVector,
    start: usize,
    end: usize,
    _meta: Object, //&'a IPersistantMap,
}

pub trait SubVector: TObject + IObj {

}


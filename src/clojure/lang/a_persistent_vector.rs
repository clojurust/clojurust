//! # Vector of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` Vector<Object> library

use core::iter::*;

// APersistentVector
use crate::clojure;
use clojure::rust::object::*;
use clojure::lang::i_persistent_vector::*;
use clojure::rust::list::*;
use clojure::rust::random_access::*;
use clojure::rust::comparable::*;
use clojure::rust::serializable::*;
use clojure::lang::i_hash_eq::*;

// RSeq
use clojure::lang::indexed_seq::*;
use clojure::rust::counted::*;
use clojure::lang::a_seq::*;

// SubVector
use clojure::lang::a_persistent_vector::*;
use clojure::lang::i_obj::*;

// Return values
use clojure::rust::obj_error::*;

pub struct SAPersistantVector<'a> {
    _hash: usize,
    _hashEq: usize,
}

pub trait APersistentVector<T>: TObject + IPersistentVector + IntoIterator 
                        + List + RandomAccess + Comparable + Seralizable
                        + IHashEq {

}

pub struct SRSeq<'a> {
    v: Object, //&'a IPersistantVector,
    i: usize,
}

pub trait RSeq: TObject + ASeq + IndexedSeq + Counted {

}

pub struct SSubVector<'a> {
    v: Object, // &'a IPersistantVector,
    start: usize,
    end: usize,
    _meta: Object, //&'a IPersistantMap,
}

pub trait SubVector: TObject + IObj {

}


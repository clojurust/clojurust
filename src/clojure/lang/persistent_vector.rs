//! # Vector of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` Vector<Object> library

// use lazy_static::__Deref;
// use std::{fmt::*, hash::*};
use std::{fmt::*};

use im::vector;

// use intertrait::cast::*;
use crate::use_obj;

use_obj! {
    clojure::rust::object;
    clojure::rust::class;
    clojure::lang::a_persistent_vector;
    clojure::lang::i_obj;
    
}

castable_to!(SPersistentVector => [sync] TObject, PersistentVector, APersistentVector);

init_obj! {
    ObjVector {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

/////////////////////////////////
// Objects
#[derive(Debug)]
pub struct SPersistentVector {
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
            inner: vector::Vector::<Object>::default()
        }
    }
}

impl Display for SPersistentVector {
    /// Return string representation of vector
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "^SObjVector {:?}", self.inner)
    }
}


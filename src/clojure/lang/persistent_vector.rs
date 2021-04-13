//! # Vector of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` Vector<Object> library

// use lazy_static::__Deref;
// use std::{fmt::*, hash::*};
use std::{fmt::*, sync::*};

use im::vector;

// use intertrait::cast::*;
use crate::use_obj;

use_obj! {
    clojure::rust::object;
    clojure::rust::class;
}

castable_to!(SAPersistentVector => [sync] TObject, APersistentVector);

init_obj! {
    ObjVector {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

#[derive(Debug)]
pub struct SAPersistentVector {
    inner: im::vector::Vector<Object>
}

impl TObject for SAPersistentVector {
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

impl Default for SAPersistentVector {
    fn default() -> Self {
        SAPersistentVector {
            inner: vector::Vector::<Object>::default()
        }
    }
}

impl Display for SAPersistentVector {
    /// Return string representation of
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "^SObjVector {:?}", self.inner)
    }
}

pub trait APersistentVector: TObject {
    fn new() -> Object
    where
        Self: Sized;
}

use crate::new_obj;

impl APersistentVector for SAPersistentVector {
    fn new() -> Object {
        new_obj!(SAPersistentVector::default())
    }
}

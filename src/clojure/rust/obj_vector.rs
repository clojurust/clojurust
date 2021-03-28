//! # Vector of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` Vector<Object> library

// use lazy_static::__Deref;
// use std::{fmt::*, hash::*};
use std::sync::*;

// use intertrait::cast::*;
use crate::use_obj;

use_obj! {
    clojure::rust::object;
    clojure::rust::class;
}

castable_to!(SObjVector => [sync] TObject, ObjVector);

init_obj! {
    ObjVector {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

pub type SObjVector = im::vector::Vector<Object>;

impl TObject for SObjVector {
    fn get_class<'a>(&self) -> &'a SClass {
        todo!()
    }

    fn to_string(&self) -> &str {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }
}

pub trait ObjVector: CastFromSync {
    fn new() -> Object
    where
        Self: Sized;
}

use crate::new_obj;

impl ObjVector for SObjVector {
    fn new() -> Object {
        new_obj!(SObjVector::default())
    }
}

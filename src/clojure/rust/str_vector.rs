//! # Vector of `Strings`s with `TObject` protocol
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

castable_to!(SStrVector => [sync] TObject, StrVector);

init_obj! {
    StrVector {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

pub type SStrVector = im::vector::Vector<String>;

castable_to!(SStrVector => [sync] TObject, StrVector);

impl TObject for SStrVector {
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

pub trait StrVector: CastFromSync {
    fn new() -> Object
    where
        Self: Sized;
}

impl StrVector {}

impl StrVector for SStrVector {
    fn new() -> Object {
        Object::new(Arc::new(SStrVector::default()))
    }
}

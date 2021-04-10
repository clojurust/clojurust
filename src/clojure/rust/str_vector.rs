//! # Vector of `Strings`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` Vector<Object> library

// use lazy_static::__Deref;
// use std::{fmt::*, hash::*};

use std::{fmt::*, sync::*};

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

#[derive(Debug)]
pub struct SStrVector { 
    inner: im::vector::Vector<String>
}

castable_to!(SStrVector => [sync] TObject, StrVector);

impl TObject for SStrVector {
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

pub trait StrVector: CastFromSync {
    fn new() -> Object
    where
        Self: Sized;
}

impl StrVector {}

use crate::new_obj;

impl Display for SStrVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "^SStrVector {:?}", self.inner)
    }
}

impl StrVector for SStrVector {
    fn new() -> Object {
        new_obj!(SStrVector::default())
    }
}

//! # HashSet of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` HashSet<Object> library

// use lazy_static::__Deref;
// use std::{fmt::*, hash::*};

use std::{fmt::*, sync::*};

// use intertrait::cast::*;

use crate::use_obj;

use_obj! {
    clojure::rust::object;
    clojure::rust::class;
}

castable_to!(SObjHashSet => [sync] TObject, ObjHashSet);

init_obj! {
    ObjHashSet {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

#[derive(Debug)]
pub struct SObjHashSet {
    inner: im::hashset::HashSet<Object>
}

impl TObject for SObjHashSet {
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

impl Display for SObjHashSet {
    /// Return string representation of
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "SObjHashSet {}", self.inner)
    }
}

pub trait ObjHashSet: CastFromSync {
    fn new() -> Object
    where
        Self: Sized;
}

use crate::new_obj;

impl ObjHashSet for SObjHashSet {
    fn new() -> Object {
        new_obj!(SObjHashSet::default())
    }
}

impl ObjHashSet {}

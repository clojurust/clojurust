//! # HashSet of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` HashSet<Object> library

// use lazy_static::__Deref;
// use std::{fmt::*, hash::*};

use std::sync::*;

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

pub type SObjHashSet = im::hashset::HashSet<Object>;

impl TObject for SObjHashSet {
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

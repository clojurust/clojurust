//! # HashMap of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` HashMap<Object,Object> library

// use intertrait::cast::*;

use crate::use_obj;
use std::sync::*;
use std::fmt::*;

use_obj! {
    clojure::rust::object;
    clojure::rust::class;
}

castable_to!(SObjHashMap => [sync] TObject, ObjHashMap);

init_obj! {
    ObjHashMap {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

#[derive(Debug)]
pub struct SObjHashMap {
    inner: im::hashmap::HashMap<Object, Object>
}

impl TObject for SObjHashMap {
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

impl Display for SObjHashMap {
    /// Return string representation of
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "SObjHashMap {}", self.inner)
    }
}

pub trait ObjHashMap: CastFromSync {
    fn new() -> Object
    where
        Self: Sized;
}

use crate::new_obj;

impl ObjHashMap for SObjHashMap {
    fn new() -> Object {
        new_obj!(SObjHashMap::default())
    }
}

impl ObjHashMap {}

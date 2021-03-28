//! # `HashMap` of `String> -> `usize` mapping for `Unique` names
//!
//! This is a wrapper on `im-rs` HashMap<String,usize> library

// use intertrait::cast::*;
use crate::use_obj;
use std::sync::*;

use_obj! {
    clojure::rust::object;
    clojure::rust::class;
}

castable_to!(SStrHashMap => [sync] TObject, StrHashMap);

init_obj! {
    StrHashMap {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

pub type SStrHashMap = im::hashmap::HashMap<String, usize>;

impl TObject for SStrHashMap {
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

pub trait StrHashMap: CastFromSync {
    fn new() -> Object
    where
        Self: Sized;
}

use crate::new_obj;

impl StrHashMap for SStrHashMap {
    fn new() -> Object {
        new_obj!(SStrHashMap::default())
    }
}

//! # HashMap of `Object`s with `TObject` protocol
//!
//! This is a wrapper on `im-rs` HashMap<Object,Object> library

use std::sync::*;

// use intertrait::cast::*;

use crate::use_obj;

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

pub type SObjHashMap = im::hashmap::HashMap<Object, Object>;

impl TObject for SObjHashMap {
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

pub trait ObjHashMap: CastFromSync {
    fn new() -> Object
    where
        Self: Sized;
}

impl ObjHashMap for SObjHashMap {
    fn new() -> Object {
        Object::new(Arc::new(SObjHashMap::default()))
    }
}

impl ObjHashMap {}

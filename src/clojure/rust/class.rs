//! # Class module
//!
//! This defines the `Class` name of the object and its `Protocol`s

use std::{fmt::Display, sync::*};

// use intertrait::cast::*;
// use intertrait::*;

/// include and init needed `Rust` `Objects` for `clojure::lang`
use crate::use_obj;
use_obj! {
    clojure::rust::object;
}

castable_to!(SClass => [sync] TObject, Class);

init_obj! {
    Class {
        clojure::rust::object::init();
    }
}

/// ## Clojure Class descriptor for Class :
/// ``` clojure
/// {
///     :name           usize
///     :protocols      #{ Protocol }
/// }
/// ```
///
#[derive(Debug)]
pub struct SClass {
    /// `usize` -> classname
    id: usize,
    /// `ObjHashMap` of `usize` -> `Protocol`s
    protocols: Object,
    /// `ObjHashMap` of `usize` -> `Member`s
    members: Object,
    /// `ObjHashMap` of `usize` -> `Member`s
    methods: Object,
}

unsafe impl Send for SClass {}

unsafe impl Sync for SClass {}

impl SClass {
    fn new(id: usize, protocols: Object, members: Object, methods: Object) -> Object {
        new_obj!(SClass {
            id,
            protocols,
            members,
            methods,
        })
    }
}

/// `Class`: `Protocol` for `Object`s and `SClass`es
///
///
pub trait Class: CastFromSync {
    /// Call `method` by id with `Object`s arguments
    fn call(&self, id: usize, args: &[Object]) -> Object;

    /// Call getter by id
    fn get(&self, id: usize) -> Object;

    /// Call setter by id
    fn set(&self, id: usize, value: Object) -> Object;
}

use crate::new_obj;

impl Class for SClass {
    /// Call named `method` with `Object`s arguments
    fn call(&self, id: usize, args: &[Object]) -> Object {
        todo!()
    }

    fn get(&self, id: usize) -> Object {
        todo!()
    }

    fn set(&self, id: usize, value: Object) -> Object {
        todo!()
    }
}

impl Display for SClass {
    /// Return string representation of
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Class")
    }
}

impl TObject for SClass {
    /// Return `Class` of `Object`
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

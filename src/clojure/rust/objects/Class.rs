//! # Class module
//!
//! This defines the `Class` name of the object and its `Protocol`s

use std::{sync::*};

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

use intertrait::*;
castable_to!(SClass => [sync] IObject, Class);

init_obj! {
    Class {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
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
pub trait Class: IObject {
    /// Call `method` by id with `Object`s arguments
    fn call(&self, obj: Object, id: usize, args: &[Object]) -> ObjResult<Object>;

    /// Call getter by id
    fn get(&self, obj: Object, id: usize) -> ObjResult<Object>;
}

impl Class for SClass {
    /// Call named `method` with `Object`s arguments
    fn call(&self, obj: Object, id: usize, args: &[Object]) -> ObjResult<Object> {
        todo!()
    }

    fn get(&self, obj: Object, id: usize) -> ObjResult<Object> {
        todo!()
    }
}

impl IObject for SClass {
    /// Return `Class` of `Object`
    fn getClass<'a>(&self) -> &'a SClass {
        todo!()
    }

    fn hashCode(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }

    fn toString(&self) -> String {
        todo!()
    }
}

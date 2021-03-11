//! Class module
//!
//! Define class of objects with
//! * Name
//! * [Protocols](crate::clojure::rust::protocol)
//!
// use std::{any::*, fmt::*, result::*};
use std::sync::*;

// use intertrait::cast::*;
use intertrait::*;

use super::obj_hashset::*;
use super::object::*;

/// ## Clojure Class descriptor for Class :
/// ``` clojure
/// {
///     :name           usize
///     :protocols      #{ Protocol }
/// }
/// ```
///
pub struct SClass {
    name: usize,
    protocols: SObjHashSet,
}

unsafe impl Send for SClass {}

unsafe impl Sync for SClass {}

castable_to!(SClass => TObject, Class);

impl SClass {
    pub fn new(name: usize, protocols: SObjHashSet) -> Object {
        Object {
            inner: Some(Arc::new(SClass { name, protocols })),
        }
    }

    /// Initialize all objects needed to create the Class interface
    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }
        INIT = true;

        println!("Class::init");

        // Insures all is initialized
        Object::init();
        ObjHashSet::init();
        // let c = Keywords::get("clojure.rust.object/Objects");
    }
}

/// `Class`: `Protocol` for `Object`s and `SClass`es
///
///
pub trait Class {
    /// Call named `method` with `Object`s arguments
    fn call(&self, name: usize, args: &[Object]) -> Object;

    /// Call getter for a named `member`
    fn get(&self, name: usize) -> Object;
}

impl Class for SClass {
    /// Call named `method` with `Object`s arguments
    fn call(&self, name: usize, args: &[Object]) -> Object {
        Object::null()
    }

    /// Call getter for a named `member`
    fn get(&self, name: usize) -> Object {
        Object::null()
    }
}

impl TObject for SClass {
    /// Return `Class` of `Object`
    fn get_class<'a>(&self) -> &'a SClass {
        todo!()
    }

    /// Return string representation of
    fn to_string(&self) -> &str {
        "Class"
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }
}

static mut INIT: bool = false;

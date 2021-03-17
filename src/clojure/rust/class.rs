//! # Class module
//!
//! This defines the `Class` name of the object and its `Protocol`s

use std::sync::*;

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
pub struct SClass {
    /// Object usize: `usize` of the classname
    name: usize,
    /// ObjHashMap of `usize` -> `Protocol`s
    protocols: Object,
    /// ObjHashMap of `usize` -> `Member`s
    members: Object,
}

unsafe impl Send for SClass {}

unsafe impl Sync for SClass {}

castable_to!(SClass => TObject, Class);

impl SClass {
    pub fn new(name: usize, protocols: Object, members: Object) -> Object {
        Object::new(Arc::new(SClass {
            name,
            protocols,
            members,
        }))
    }

    // pub fn o_new_o(name: Object, protocols: Object, members: Object) -> Object {
    //     let a = Object::inn::<Usize>(name);
    //     Object::new::<SClass>(SClass::new(name, protocols, members))
    // }
}

/// `Class`: `Protocol` for `Object`s and `SClass`es
///
///
pub trait Class {
    /// Call `method` by id with `Object`s arguments
    fn call(&self, name: usize, args: &[Object]) -> Object;

    /// Call getter by id
    fn get(&self, name: usize) -> Object;

    /// Call setter by id
    fn set(&self, name: usize, value: Object) -> Object;
}

impl Class for SClass {
    /// Call named `method` with `Object`s arguments
    fn call(&self, name: usize, args: &[Object]) -> Object {
        todo!()
    }

    fn get(&self, name: usize) -> Object {
        todo!()
    }

    fn set(&self, name: usize, value: Object) -> Object {
        todo!()
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

//! clojure::rust::class: Define class of objects

// use std::{any::*, fmt::*, result::*};
use std::sync::*;

// use intertrait::cast::*;
use intertrait::*;

use super::object::*;
use super::phashmap::*;

/// ## Clojure Class descriptor for Class :
/// ``` clojure
/// {
///     :super_class    Class
///     :protocols      [
///                        ... Protocols
///                     ]
///     :get            {
///                         :name            :clojure.rust.class/Class
///                         ... added from super-class
///                     }
///     :methods        {
///                         ... added from super-class
///                         ... added from protocols
///                     }
/// }
/// ```
///
/// ## Rust Class descriptor for Class :
/// ``` rust
/// pub struct Class {
///     const CLASS_NAME = "",
///     pub super_class: SObject, // Class
///     pub protocols: SObject,   // HashSet of Protocols
///     pub get: SObject,         // HashMap of Getters
///     pub methods: SObject,     // HashMap of Methods
///     pub functions: SObject,   // HashMap of static functions
/// }
/// ```
pub struct SClass {
    inner: SPHashMap,
}

unsafe impl Send for SClass {}

unsafe impl Sync for SClass {}

castable_to!(SClass => TObject);

impl SClass {
    pub fn new(inner: SPHashMap) -> Object {
        Object {
            inner: Some(Arc::new(SClass { inner })),
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
        SPHashMap::init();
        // let c = Keywords::get("clojure.rust.object/Objects");
    }
}

/// `Object` `Protocol` for all defined `Object`s
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

//! clojure::rust::class: Define class of objects

use intertrait::cast::*;
use intertrait::*;
use std::sync::Arc;
use std::{any::*, fmt::*, result::*, sync::*};

use super::object::*;
use super::phashmap::*;
use super::stri::*;
use super::unique::*;

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
        Unique::init();
        Object::init();
        SPHashMap::init();
        // let c = Keywords::get("clojure.rust.object/Objects");
    }
}

trait Class {
    fn call(obj: &TObject, name: &Object, args: &[Object]) -> Object;

    fn implementation(&self, name: &Object) -> Object;
}

impl Class for SClass {
    fn call(obj: &TObject, name: &Object, args: &[Object]) -> Object {
        todo!()
    }

    fn implementation(&self, name: &Object) -> Object {
        todo!()
    }
}

impl TObject for SClass {
    /// Return `Class` of `Object`
    fn get_class(&self) -> &SClass {
        todo!()
    }

    /// Call named `method` with `Object`s arguments
    fn call(&self, name: usize, args: &[Object]) -> Object {
        Object::null()
    }

    /// Call getter for a named `member`
    fn get(&self, name: usize) -> Object {
        Object::null()
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

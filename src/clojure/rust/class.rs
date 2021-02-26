//! clojure::rust::class: Define class of objects

use intertrait::cast::*;
use intertrait::*;
use std::sync::Arc;
use std::{any::*, fmt::*, result::*, sync::*};

use super::object::*;
use super::phashmap::*;
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
///     pub super_class: SObject, // Class
///     pub protocols: SObject,   // HashSet of Protocols
///     pub get: SObject,         // HashMap of Getters
///     pub methods: SObject,     // HashMap of Methods
///     pub functions: SObject,   // HashMap of static functions
/// }
/// ```
pub struct Class {
    inner: PHashMap,
}

castable_to!(Class => Object);

impl Class {
    pub fn new(inner: PHashMap) -> SObject {
        SObject {
            inner: Some(Arc::new(Class { inner })),
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
        SObject::init();
        PHashMap::init();
        // let c = Keywords::get("clojure.rust.object/Objects");
    }
}

impl Object for Class {
    /// Return `Class` of `Object`
    fn get_class<'a>(&'a self) -> &'a Class {
        todo!()
    }

    /// Call named `method` with `Object`s arguments
    fn call(&self, name: &str, args: &[SObject]) -> SObject {
        SObject::null()
    }

    /// Call getter for a named `member`
    fn get(&self, name: &str) -> SObject {
        SObject::null()
    }

    /// Return string representation of
    fn to_string(&self) -> String {
        String::from("class")
    }

    fn get_hash(&self) -> usize {
        0
    }
}

static mut INIT: bool = false;

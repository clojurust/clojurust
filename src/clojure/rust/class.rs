//! clojure::rust::class: Define class of objects

use intertrait::cast::*;
use intertrait::*;
use std::sync::Arc;
use std::{any::*, fmt::*, result::*, sync::*};

use super::object;
use super::phashmap;

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
    inner: phashmap::SPHashMap,
}

unsafe impl Send for SClass {}

unsafe impl Sync for SClass {}

castable_to!(SClass => object::TObject);

impl SClass {
    pub fn new(inner: phashmap::SPHashMap) -> object::Object {
        object::Object {
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
        object::Object::init();
        phashmap::SPHashMap::init();
        // let c = Keywords::get("clojure.rust.object/Objects");
    }
}

trait Class {
    fn call(
        obj: &object::TObject,
        name: &object::Object,
        args: &[object::Object],
    ) -> object::Object;

    fn implementation(&self, name: &object::Object) -> object::Object;
}

impl Class for SClass {
    fn call(
        obj: &object::TObject,
        name: &object::Object,
        args: &[object::Object],
    ) -> object::Object {
        todo!()
    }

    fn implementation(&self, name: &object::Object) -> object::Object {
        todo!()
    }
}

impl object::TObject for SClass {
    /// Return `Class` of `Object`
    fn get_class(&self) -> &SClass {
        todo!()
    }

    /// Call named `method` with `Object`s arguments
    fn call(&self, name: usize, args: &[object::Object]) -> object::Object {
        object::Object::null()
    }

    /// Call getter for a named `member`
    fn get(&self, name: usize) -> object::Object {
        object::Object::null()
    }

    /// Return string representation of
    fn to_string(&self) -> &str {
        "Class"
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &object::Object) -> bool {
        todo!()
    }
}

static mut INIT: bool = false;

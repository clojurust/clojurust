//! clojure::rust::class: Define class of objects

pub use crate::clojure;
use clojure::core::object::Object;
use clojure::core::keywords::*;
use clojure::lang::hashmap::HashMap;

pub struct Class {
    map: Object,
}

/// Class descriptor for Class :
/// ``` clojure
/// {
///     :super_class    Class
///     :protocols      [
///                        ... Protocols
///                     ]
///     :members        {
///                         :name            :clojure.rust.class/Class
///                         ... added from super-class
///                     }
///     :methods        {
///                         ... added from super-class
///                         ... added from protocols
///                     }
/// }
/// ```



impl Class {
    pub fn new(name: String, map: Object) -> Object {
        let ob = 
                Class {
                    map:        map.clone(),
                };
        Object::new::<Class>(Keywords::get(name, *CORE),
                                Box::new(ob)).clone()
    }
    

    /// Initialize all objects needed to create the Class interface
    pub unsafe fn init() {
        // only execute one time
        if INIT {return;}
        INIT = true;
        
        println!("Class::init");

        // Insures all is initialized
        Keywords::init();
        Object::init();
        HashMap::init();
        // let c = Keywords::get("clojure.rust.object/Objects");

    }
}

static INIT: bool = false;

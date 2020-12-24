use im::hashmap;

pub use crate::clojure;
use clojure::core::object::*;

pub struct HashMap {
    map: hashmap::HashMap<Object, Object>,
}

impl HashMap {
    pub unsafe fn init() {
        // only execute one time
        if INIT {return;}
        INIT = true;

        // Insures all is initialized
        Object::init();
    }
}

static INIT: bool = false;
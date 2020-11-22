//! clojure::rust::class: Define class of objects
use im::vector::Vector;
use im::hashmap::HashMap;
use std::sync::*;
// use lazy_static::lazy_static;
use super::function::*;
// use super::keywords::*;

pub struct Class {
    class_name: usize, // Keyword id of name of the class. Key
    super_class: usize, // Keyword id of super-class
    protocols: Vector<usize>, // List of protocols
    members: HashMap<usize, usize>, // List of members (name: Keyword (usize), class<usize)) 
    methods: HashMap<usize, Arc<Function>>, // List of object functions
    functions: HashMap<usize, Arc<Function>>, // Static functions
}

pub struct Classes {
    classes: HashMap<usize, Arc<Class>>,
}

impl Classes {
    pub fn new() -> Classes {
        Classes {
            classes: HashMap::<usize, Arc<Class>>::new(),
        }
    }
}

/* Test with RwLock for now, change to 
lazy_static! {
    static ref CLASSES: Arc<RwLock<Classes>> = Arc::new(RwLock::new(Classes::new()));
}
*/

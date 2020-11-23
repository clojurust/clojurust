//! clojure::rust::class: Define class of objects
use im::vector::Vector;
use im::hashmap::HashMap;
use std::sync::*;
use super::function::*;
// use super::keywords::*;

#[allow(dead_code)]
pub struct Class {
    class_name: usize, // Keyword id of name of the class. Key
    super_class: usize, // Keyword id of super-class
    protocols: Vector<usize>, // List of protocols
    members: HashMap<usize, usize>, // List of members (name: Keyword (usize), class<usize)) 
    methods: HashMap<usize, Arc<Function>>, // List of object functions
    functions: HashMap<usize, Arc<Function>>, // Static functions
}

#[allow(dead_code)]
pub struct Classes {
    classes: HashMap<usize, Arc<Class>>,
}

impl<'i> Classes {
    #[allow(dead_code)]
    pub fn new() -> Classes {
        Classes {
            classes: HashMap::<usize, Arc<Class>>::new(),
        }
    }
}


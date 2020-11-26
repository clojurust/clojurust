//! clojure::rust::class: Define class of objects

use im::vector::Vector;
use im::hashmap::HashMap;
use super::function::*;
use super::member::*;

pub struct Class {
    class_name: usize, // Keyword id of name of the class. Key
    super_class: usize, // Keyword id of super-class
    protocols: Vector<usize>, // List of protocols
    members: HashMap<usize, Member>, // List of members (name: Keyword (usize), class<usize)) 
    methods: HashMap<usize, Function>, // List of object functions
}

impl Class {
    pub fn new() -> Class {
        Class {
            class_name: 9, // Keyword id of name of the class. Key
            super_class: 0, // Keyword id of super-class
            protocols: Vector::<usize>::new(), // List of protocols
            members: HashMap::<usize, Member>::new(), // List of members (name: Keyword (usize), class<usize)) 
            methods: HashMap::<usize, Function>::new(), // List of object functions
        }        
    } 
}


//! clojure::rust::class: Define class of objects

use im::hashmap::HashMap;
use super::object::*;
pub struct Class {
    super_class: Object, // Keyword id of super-class
    protocols: HashMap<Object, Object>, // List of protocols
    members: HashMap<Object, Object>, // List of members (name: Keyword (usize), class<usize)) 
    methods: HashMap<Object, Object>, // List of object functions
}

impl Class {
    pub fn new() -> Class {
        Class {
            super_class: , // Keyword id of super-class
            protocols: HashMap::<Object, Object>::new(), // List of Protocol
            members: HashMap::<Object, Object>::new(), // List of Members (name: Keyword (usize), class<usize)) 
            methods: HashMap::<Object, Object>::new(), // List of Functions
        }        
    } 
}

pub fn init() {

}

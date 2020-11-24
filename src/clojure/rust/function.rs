//! Anonymous Function with multi-arity

// use im::vector::Vector;
use im::hashmap::HashMap;
// use std::sync::*;
// use lazy_static::lazy_static;
use clojure::rust::implementation::*;

pub struct Function {
    pub higher: Option<usize>, // optional maximum arity
    pub func: HashMap<usize, Implementation>, // all implementations
}

impl<'i> Function {
        pub fn new() -> Function {
        Function {
            higher: None,
            func: HashMap::<usize, Implementation>::new(),
        }
    }

        pub fn get(&self, arity: usize) -> Option<&Implementation> {
        match self.higher {
            Some(max) => {
                if arity > max {
                    if let Some(implem) = self.func.get(&max) {
                        if implem.multiary {
                            Some(implem)
                        } else {
                            None
                        }
                    }
                    else {
                        None
                    }
                }
                else {
                    self.func.get(&max)
                }
            }

            // If no max => no implementation
            None => None
        }
    }
}


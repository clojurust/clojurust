//! Anonymous Function with multi-arity

use im::hashmap::HashMap;
pub use crate::clojure;
use clojure::core::object::*;
use clojure::core::rust_obj::*;

pub struct Function {
    pub higher: Option<usize>,
    pub func: HashMap<usize, Object>, // all implementations
}

impl Function {
    pub fn new() -> Function {
        Function {
            higher: None,
            func: HashMap::<usize, Object>::new(),
        }
    }

    pub fn get(&self, arity: usize) -> Object {
        match self.higher {
            Some(max) => {
                if arity > max {
                    let implem = self.func.get(&max).unwrap().clone();
                    if implem.multiary {
                        implem.clone()
                    } 
                    else {
                        RustObj::null()
                    }
                }
                else {
                    let implem = self.func.get(&arity);
                    match implem {
                        Some(res) => res.clone(),
                        None => RustObj::null(),
                    }
                }
            }

            // If no max => no implementation
            None => RustObj::null()
        }
    }
}

pub fn init() {

}

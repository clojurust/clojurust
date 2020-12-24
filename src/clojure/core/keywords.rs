//! clojure::rust::keyword: keyword index

use im::*;
use im::hashmap::HashMap;
use std::sync::*;
use lazy_static::lazy_static;

/// A keyword storage structure
///
/// We will store all `String`s used as reference to objects as `usize`.
/// `usize` values are unique and immutable for every `String`.
/// `Strings` are added incrementally to the `vect` `Vector` and cannot be destroyed.
/// as a `String` is added, it's index is added in the `map` `HashMap`.
/// # Examples
/// ```
///
#[derive(Default)]
#[derive(Debug)]
pub struct Keywords {
    map: HashMap<String, usize>,
    vect: Vector<String>,
}

impl Keywords {
    pub fn new() -> Keywords {
        Keywords {
            map: HashMap::<String, usize>::new(),
            vect: Vector::<String>::new(),
        }
    }

    pub fn current(keywords: RwLock<Arc<Keywords>>) -> Arc<Keywords> {
        keywords.read().unwrap().clone()
    }

    pub fn current_mut(keywords: RwLock<Arc<Keywords>>) -> Arc<Keywords> {
        keywords.write().unwrap().clone()
    }

    pub fn make_current(self, keywords: RwLock<Arc<Keywords>>) {
        *keywords.write().unwrap() = Arc::new(self);
    }

    pub fn len(keywords: RwLock<Arc<Keywords>>) -> usize {
        Keywords::current(keywords).vect.len()
    }

    pub fn get(key: String, keywords: RwLock<Arc<Keywords>>) -> usize {
        let i = Keywords::current(keywords).clone();
        let a = i.as_ref();
        let mut m = a.map.clone();
        let mut v = a.vect.clone();
        let length = Keywords::len(keywords);

        match m.get(&key) {
            // found entry
            Some(idx) => *idx,

            // Not found: add entry in vect and map
            None => {
                // Insert new values in vector and map
                &v.push_back(*&key);
                &m.update(*&key, *&length);

                let k = Keywords {
                    map: m,
                    vect: v,
                };
                k.make_current(keywords);

                // return new index that was length of vector
                length
            }
        }
    }

    pub fn test(key: String, keywords: RwLock<Arc<Keywords>>) -> bool {
        let i = Keywords::current(keywords).clone();
        let a = i.as_ref();
        match a.map.get(&key) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn to_string(index: usize, keywords: RwLock<Arc<Keywords>>) -> String {
        match Keywords::current(keywords).vect.get(index) {
            Some(key) => { String::from(key) }
            None => {String::from("")}
        }
    }

    pub unsafe fn init() {
    }
}

impl Drop for Keywords {
    fn drop(&mut self) {
        println!("Dropping Keyword state! -> {:?}", self);
    }
}

static INIT: bool = false;

pub fn init_keywords() -> RwLock<Arc<Keywords>> {
    RwLock::new(Arc::new(Default::default()))
}

lazy_static! {
    /// Private access to static global `Keywords` struture.
    ///
    /// Here will be stored and retrived keywords data.
    pub static ref KEYWORDS: RwLock<Arc<Keywords>> = init_keywords();
    pub static ref CORE: RwLock<Arc<Keywords>> = init_keywords();
}

#[test]
fn test_keywords() {
    // Initial state
    println!("Init state len = {:?} state = {:?}", Keywords::len(*CORE), Keywords::current(*CORE));
    
    let e1 = Keywords::current(*CORE);

    // Call init_static
    println!("New state len = {:?} state = {:?}", Keywords::len(*CORE), Keywords::current(*CORE));

    let e2 = Keywords::current(*CORE);

    // add first keyword
    let o = Keywords::get(String::from("essai"), *CORE);
    println!("add essai len = {:?} state = {:?}", Keywords::len(*CORE), Keywords::current(*CORE));

    let e3 = Keywords::current(*CORE);

    // add second keyword
    Keywords::get("essai2".to_string(), *CORE);
    println!("add essai2 len = {:?} state = {:?}", Keywords::len(*CORE), Keywords::current(*CORE));

    let e4 = Keywords::current(*CORE);

    // display existing keywords                                
    println!("Keyword 0 = \"{}\"", Keywords::to_string(0, *CORE));
    println!("Keyword 1 = \"{}\"", Keywords::to_string(1, *CORE));

    // display inexisting keyword -> ""
    println!("Keyword 2 = \"{}\"", Keywords::to_string(2, *CORE));

    // Verify persistant state
    println!("State 1 = {:?}", e1);
    println!("State 2 = {:?}", e2);
    println!("State 3 = {:?}", e3);
    println!("State 4 = {:?}", e4);

    assert_eq!(1,1)

    // display of droppings
    // At the output state 1 and 2 are the same and so only one drop,
    // event is there are two Arc, but it's a lone struct.
    // State 3 is droped as it's the output of test function, and 
    // e3 is the only link to the state.
    // As state 4 is linked in the KEYWORDS global variable, the drop
    // is not displayed..
}


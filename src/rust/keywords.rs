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

    pub fn current() -> Arc<Keywords> {
        KEYWORDS.read().unwrap().clone()
    }

    pub fn current_mut() -> Arc<Keywords> {
        KEYWORDS.write().unwrap().clone()
    }

    pub fn make_current(self) {
        *KEYWORDS.write().unwrap() = Arc::new(self);
    }

    pub fn len() -> usize {
        Keywords::current().vect.len()
    }

    pub fn get(key: String) -> usize {
        let i = Keywords::current().clone();
        let a = i.as_ref();
        let mut m = a.map.clone();
        let mut v = a.vect.clone();
        let length = Keywords::len();

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
                k.make_current();

                // return new index that was length of vector
                length
            }
        }
    }

    pub fn test(key: String) -> bool {
        let i = Keywords::current().clone();
        let a = i.as_ref();
        match a.map.get(&key) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn to_string(index: usize) -> String {
        match Keywords::current().vect.get(index) {
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
}

#[test]
fn test_keywords() {
    // Initial state
    println!("Init state len = {:?} state = {:?}", Keywords::len(), Keywords::current());
    
    let e1 = Keywords::current();

    // Call init_static
    println!("New state len = {:?} state = {:?}", Keywords::len(), Keywords::current());

    let e2 = Keywords::current();

    // add first keyword
    let o = Keywords::get(String::from("essai"));
    println!("add essai len = {:?} state = {:?}", Keywords::len(), Keywords::current());

    let e3 = Keywords::current();

    // add second keyword
    Keywords::get("essai2".to_string());
    println!("add essai2 len = {:?} state = {:?}", Keywords::len(), Keywords::current());

    let e4 = Keywords::current();

    // display existing keywords                                
    println!("Keyword 0 = \"{}\"", Keywords::to_string(0));
    println!("Keyword 1 = \"{}\"", Keywords::to_string(1));

    // display inexisting keyword -> ""
    println!("Keyword 2 = \"{}\"", Keywords::to_string(2));

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


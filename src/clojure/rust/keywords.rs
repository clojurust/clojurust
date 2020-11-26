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
/// # Exxamples
/// ```
///
#[derive(Default)]
struct Keywords {
    map: RwLock<HashMap<String, usize>>,
    vect: RwLock<Vector<String>>,
}

impl Keywords {
    pub fn current() -> Arc<Keywords> {
        KEYWORDS.read().unwrap().clone()
    }

    pub fn make_current(self) {
        *KEYWORDS.write().unwrap() = Arc::new(self);
    }

    fn new() -> Keywords {
        Keywords {
            map: HashMap::<String, usize>::new(),
            vect: Vector::<String>::new(),
        }
    }

    pub fn len() -> usize {
        KEYWORDS.read().unwrap().vect.len().clone()
    }

    pub fn get(key: &str) -> usize {
        let a = RwLock::new(RustObj::current();
        let m = a
        // warning len cannot be called when there's a lockWrite on keyword
        // let length = Keywords::len(); // can be false as out of the write lock
        let k = Arc::clone(KEYWORDS);
        let mut m = k.write().unwrap();

        // so we get the length from LockWrite
        let length = m.vect.len();

        match m.map.get(key) {
            // found entry
            Some(idx) => { *idx }

            // Not found: add entry in vect and map
            None => {
                m.vect.insert(length, String::from(key));
                m.map.insert(String::from(key), length);
                
                // return new index that was length of vector
                length
            }
        }
    }

    pub fn to_string(index: usize) -> String {
        match KEYWORDS.read().unwrap().vect.get(index) {
            Some(key) => { String::from(key) }
            None => {String::from("")}
        }
    }

    pub fn init_static() {
        println!("init static");
    }
}

lazy_static! {
    /// Private access to static global `Keywords` struture.
    ///
    /// Here will be stored and retrived keywords data.
    pub static ref KEYWORDS: RwLock<Arc<Keywords>> = 
                        RwLock::new(Arc::new(Keywords::new()));
}

#[test]
fn test_the_thing() {
    println!("Init state {:?}", Keywords::len());
    Keywords::init_static(); // expected to succeed
    println!("New state {:?}", Keywords::len());
    Keywords::get("essai");
    println!("add essai {:?} {}", Keywords::len(), 
                                    Keywords::to_string(0));
    Keywords::get("essai2");
    println!("add essai2 {:?} {} {}", Keywords::len(), 
                                    Keywords::to_string(0),
                                    Keywords::to_string(1));
    assert_eq!(1,1)
}
